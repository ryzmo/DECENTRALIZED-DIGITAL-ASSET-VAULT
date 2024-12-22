use candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, query, update};
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
#[init]
fn init() {
    ic_cdk::println!("Decentralized Digital Asset Vault initialized!");
}

// Tambahkan aset baru
#[update]
fn add_asset(owner: UserId, name: String, content: Vec<u8>) -> AssetId {
    let asset_id = NEXT_ASSET_ID.with(|id| {
        let mut id = id.borrow_mut();
        let current_id = *id;
        *id += 1;
        current_id
    });

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
    asset_id
}

// Berikan akses ke pengguna lain
#[update]
fn share_asset(asset_id: AssetId, owner: UserId, recipient: UserId) -> Result<String, String> {
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
#[query]
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

    // Tambahkan log akses di luar peminjaman
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
#[query]
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
