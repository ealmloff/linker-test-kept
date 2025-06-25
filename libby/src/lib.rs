use dioxus::prelude::*;

#[used]
static _EXTERNAL_ASSET: Asset = asset!(
    "/66571940.jpeg",
    ImageAssetOptions::new()
        .into_asset_options()
        .with_hash_suffix(false)
);

pub fn please_build_me() {
    println!("Please build me!");
}

// The used attribute cannot be applied to extern "C" functions
// If you dump the symbols with objdump this symbol does not show up
const __LEN: usize = 10;
#[unsafe(no_mangle)]
extern "C" fn generate_link_section() -> *const u8 {
    &[0u8; __LEN] as &[_] as *const [_] as *const _
}
