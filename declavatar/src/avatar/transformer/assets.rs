use crate::{
    avatar::{
        data::{Asset, AssetType},
        transformer::{failure, success, Compiled, Context, LogKind},
    },
    decl::data::{AssetKey as DeclAssetKey, AssetType as DeclAssetType, Assets as DeclAssets},
};

pub fn compile_assets_blocks(
    ctx: &mut Context,
    assets_blocks: Vec<DeclAssets>,
) -> Compiled<Vec<Asset>> {
    let mut assets: Vec<Asset> = vec![];

    let decl_assets = assets_blocks.into_iter().flat_map(|ab| ab.assets);
    for decl_asset in decl_assets {
        let Some(asset) = compile_asset(ctx, decl_asset, &assets) else {
            continue;
        };
        assets.push(asset);
    }

    success(assets)
}

fn compile_asset(
    ctx: &mut Context,
    decl_asset: DeclAssetKey,
    declared: &[Asset],
) -> Compiled<Asset> {
    let key = decl_asset.key;
    let asset_type = match decl_asset.ty {
        DeclAssetType::Material => AssetType::Material,
        DeclAssetType::Animation => AssetType::Animation,
        DeclAssetType::Indeterminate => {
            ctx.log_error(LogKind::IndeterminateAsset(key));
            return failure();
        }
    };

    if let Some(defined) = declared.iter().find(|a| a.key == key) {
        if defined.asset_type != asset_type {
            ctx.log_error(LogKind::IncompatibleAssetDeclaration(key));
        }
        return failure();
    }

    success(Asset { asset_type, key })
}
