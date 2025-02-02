use crate::common::errors::RustCError;
use crate::common::types::{Ptr, PtrBytes, PtrString};
use crate::common::ur::UREncodeResult;
use crate::common::utils::recover_c_char;
use alloc::slice;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use app_monero::{address::Address, key::*, structs::*, utils::constants::PRVKEY_LEH, utils::*};
use cty::uint32_t;
use serde_json::{json, Value};
use ur_registry::crypto_hd_key::CryptoHDKey;
use ur_registry::error::URError;
use ur_registry::traits::RegistryItem;

fn safe_parse_key(decrypt_key: PtrBytes) -> Result<[u8; PRVKEY_LEH], RustCError> {
    let decrypt_key = unsafe { slice::from_raw_parts(decrypt_key, PRVKEY_LEH) };

    if decrypt_key.len() != PRVKEY_LEH {
        return Err(RustCError::InvalidMasterFingerprint);
    }

    Ok(decrypt_key.try_into().unwrap())
}

fn safe_parse_pincode(pincode: PtrBytes) -> Result<[u8; 6], RustCError> {
    let pincode = unsafe { slice::from_raw_parts(pincode, 6) };

    if pincode.len() != 6 {
        return Err(RustCError::InvalidMasterFingerprint);
    }

    Ok(pincode.try_into().unwrap())
}

fn generate_wallet_result(
    primaryAddress: String,
    privateViewKey: String,
    is_encrypted: bool,
) -> UREncodeResult {
    let result = json!({
        "version": 0,
        "primaryAddress": primaryAddress,
        "privateViewKey": privateViewKey,
        "restoreHeight": 0,
        "encrypted": is_encrypted,
        "source": "Keystone"
    });

    UREncodeResult::text(result.to_string())
}

#[no_mangle]
pub extern "C" fn get_connect_cake_wallet_ur(
    pub_spend_key: PtrString,
    private_view_key: PtrString,
) -> *mut UREncodeResult {
    let spend_key = recover_c_char(pub_spend_key);
    let pvk = hex::decode(recover_c_char(private_view_key)).unwrap();

    let primary_address = Address::new(
        Network::Mainnet,
        AddressType::Standard,
        PublicKey::from_str(spend_key.as_str()).unwrap(),
        PrivateKey::from_bytes(&pvk).get_public_key(),
    );

    generate_wallet_result(
        primary_address.to_string(),
        hex::encode(pvk.to_vec()),
        false,
    )
    .c_ptr()
}

#[no_mangle]
pub extern "C" fn get_connect_cake_wallet_ur_encrypted(
    pub_spend_key: PtrString,
    private_view_key: PtrString,
    pincode: PtrBytes,
) -> *mut UREncodeResult {
    let spend_key = recover_c_char(pub_spend_key);
    let pvk = hex::decode(recover_c_char(private_view_key)).unwrap();
    let pincode = match safe_parse_pincode(pincode) {
        Ok(pincode) => pincode,
        _ => return UREncodeResult::from(RustCError::InvalidXPub).c_ptr(),
    };

    let primary_address = Address::new(
        Network::Mainnet,
        AddressType::Standard,
        PublicKey::from_str(spend_key.as_str()).unwrap(),
        PrivateKey::from_bytes(&pvk).get_public_key(),
    );

    let data_encrypt_wrapper =
        |data: String| -> String { hex::encode(encrypt_data_with_pincode(data, pincode)) };

    generate_wallet_result(
        data_encrypt_wrapper(primary_address.to_string()),
        data_encrypt_wrapper(hex::encode(pvk.to_vec())),
        true,
    )
    .c_ptr()
}
