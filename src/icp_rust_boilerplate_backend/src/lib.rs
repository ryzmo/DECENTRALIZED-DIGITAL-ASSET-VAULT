use candid::{CandidType, Deserialize};
use std::collections::HashMap;

// Tipe untuk Asset ID dan User ID
type AssetId = u64;
type UserId = String;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct AccessLog {
    user_id: UserId,
    timestamp: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct DigitalAsset {
    id: AssetId,
    owner: UserId,
    name: String,
    content: Vec<u8>, // Data terenkripsi
    shared_with: Vec<UserId>, // Daftar pengguna yang diberikan izin
    access_log: Vec<AccessLog>, // Riwayat akses
}

// Penyimpanan aset digital dan ID aset berikutnya
thread_local! {
    static ASSETS: std::cell::RefCell<HashMap<AssetId, DigitalAsset>> = std::cell::RefCell::new(HashMap::new());
    static NEXT_ASSET_ID: std::cell::RefCell<AssetId> = std::cell::RefCell::new(1);
}

// Fungsi inisialisasi
#[ic_cdk::init]
fn init() {
    ic_cdk::println!("Decentralized Digital Asset Vault initialized!");
}

// Fungsi utilitas untuk validasi input
fn validate_fields(fields: Vec<(&str, &str)>) -> Result<(), String> {
    for (field_name, field_value) in fields {
        if field_value.trim().is_empty() {
            return Err(format!("Field '{}' cannot be empty.", field_name));
        }
    }
    Ok(())
}

// Fungsi utilitas untuk ID aset unik
fn generate_asset_id() -> AssetId {
    NEXT_ASSET_ID.with(|id| {
        let mut id = id.borrow_mut();
        let current_id = *id;
        *id += 1;
        current_id
    })
}

// Tambahkan aset baru
#[ic_cdk::update]
fn add_asset(owner: UserId, name: String, content: Vec<u8>) -> Result<AssetId, String> {
    validate_fields(vec![("owner", &owner), ("name", &name)])?;
    if content.is_empty() {
        return Err("Content cannot be empty.".to_string());
    }

    let asset_id = generate_asset_id();

    let asset = DigitalAsset {
        id: asset_id,
        owner: owner.clone(),
        name,
        content,
        shared_with: vec![],
        access_log: vec![],
    };

    ASSETS.with(|assets| {
        assets.borrow_mut().insert(asset_id, asset);
    });

    ic_cdk::println!("Asset added by user: {}", owner);
    Ok(asset_id)
}

// Berikan akses ke pengguna lain
#[ic_cdk::update]
fn share_asset(asset_id: AssetId, owner: UserId, recipient: UserId) -> Result<String, String> {
    validate_fields(vec![("owner", &owner), ("recipient", &recipient)])?;
    if owner == recipient {
        return Err("Owner cannot share the asset with themselves.".to_string());
    }

    ASSETS.with(|assets| {
        let mut assets = assets.borrow_mut();
        if let Some(asset) = assets.get_mut(&asset_id) {
            if asset.owner == owner {
                if !asset.shared_with.contains(&recipient) {
                    asset.shared_with.push(recipient.clone());
                }
                Ok(format!("Asset shared with user: {}", recipient))
            } else {
                Err("Only the owner can share the asset.".to_string())
            }
        } else {
            Err("Asset not found.".to_string())
        }
    })
}

// Lihat detail aset
#[ic_cdk::query]
fn get_asset(asset_id: AssetId, user_id: UserId) -> Result<(String, Vec<u8>), String> {
    let result = ASSETS.with(|assets| {
        let assets = assets.borrow();
        if let Some(asset) = assets.get(&asset_id) {
            if asset.owner == user_id || asset.shared_with.contains(&user_id) {
                Ok((asset.name.clone(), asset.content.clone()))
            } else {
                Err("Access denied.".to_string())
            }
        } else {
            Err("Asset not found.".to_string())
        }
    });

    if let Ok(_) = result {
        ASSETS.with(|assets| {
            let mut assets = assets.borrow_mut();
            if let Some(asset) = assets.get_mut(&asset_id) {
                asset.access_log.push(AccessLog {
                    user_id: user_id.clone(),
                    timestamp: ic_cdk::api::time(),
                });
            }
        });
    }

    result
}

// Lihat riwayat akses aset
#[ic_cdk::query]
fn get_access_log(asset_id: AssetId, owner: UserId) -> Result<Vec<AccessLog>, String> {
    ASSETS.with(|assets| {
        let assets = assets.borrow();
        if let Some(asset) = assets.get(&asset_id) {
            if asset.owner == owner {
                Ok(asset.access_log.clone())
            } else {
                Err("Only the owner can view the access log.".to_string())
            }
        } else {
            Err("Asset not found.".to_string())
        }
    })
}

// Fungsi tambahan: Hapus aset
#[ic_cdk::update]
fn delete_asset(asset_id: AssetId, owner: UserId) -> Result<String, String> {
    ASSETS.with(|assets| {
        let mut assets = assets.borrow_mut();
        if let Some(asset) = assets.get(&asset_id) {
            if asset.owner == owner {
                assets.remove(&asset_id);
                Ok("Asset deleted successfully.".to_string())
            } else {
                Err("Only the owner can delete the asset.".to_string())
            }
        } else {
            Err("Asset not found.".to_string())
        }
    })
}

// Fungsi tambahan: Cabut akses pengguna
#[ic_cdk::update]
fn revoke_access(asset_id: AssetId, owner: UserId, recipient: UserId) -> Result<String, String> {
    ASSETS.with(|assets| {
        let mut assets = assets.borrow_mut();
        if let Some(asset) = assets.get_mut(&asset_id) {
            if asset.owner == owner {
                if asset.shared_with.contains(&recipient) {
                    asset.shared_with.retain(|user| user != &recipient);
                    Ok(format!("Access revoked for user: {}", recipient))
                } else {
                    Err("User does not have access to this asset.".to_string())
                }
            } else {
                Err("Only the owner can revoke access.".to_string())
            }
        } else {
            Err("Asset not found.".to_string())
        }
    })
}

// Fungsi tambahan: Daftar semua aset milik pengguna
#[ic_cdk::query]
fn list_assets_by_owner(owner: UserId) -> Result<Vec<DigitalAsset>, String> {
    ASSETS.with(|assets| {
        let assets = assets.borrow();
        let user_assets: Vec<DigitalAsset> = assets.values()
            .filter(|asset| asset.owner == owner)
            .cloned()
            .collect();

        if user_assets.is_empty() {
            Err("No assets found for this owner.".to_string())
        } else {
            Ok(user_assets)
        }
    })
}

// Fungsi tambahan: Daftar semua aset yang dibagikan dengan pengguna
#[ic_cdk::query]
fn get_shared_assets(user_id: UserId) -> Result<Vec<DigitalAsset>, String> {
    ASSETS.with(|assets| {
        let assets = assets.borrow();
        let shared_assets: Vec<DigitalAsset> = assets.values()
            .filter(|asset| asset.shared_with.contains(&user_id))
            .cloned()
            .collect();

        if shared_assets.is_empty() {
            Err("No shared assets found for this user.".to_string())
        } else {
            Ok(shared_assets)
        }
    })
}

ic_cdk::export_candid!();