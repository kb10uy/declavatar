use crate::{
    avatar_v2::{
        data::asset::{Asset, AssetType},
        logger::{Log, Logger, LoggerContext},
        transformer::{failure, success, Compiled},
    },
    decl_v2::data::asset::{DeclAsset, DeclAssets},
};

pub fn compile_assets_blocks(
    ctx: &mut Logger,
    assets_blocks: Vec<DeclAssets>,
) -> Compiled<Vec<Asset>> {
    #[derive(Debug)]
    pub struct Context(usize);
    impl LoggerContext for Context {
        fn write_context(&self, inner: String) -> String {
            format!("assets block {} > {}", self.0, inner)
        }
    }

    let mut assets = vec![];
    for (index, decl_assets) in assets_blocks.into_iter().enumerate() {
        ctx.push_context(Context(index));
        for decl_asset in decl_assets.assets {
            let Some(asset) = compile_asset(ctx, decl_asset, &assets) else {
                continue;
            };
            assets.push(asset);
        }
        ctx.pop_context();
    }

    success(assets)
}

fn compile_asset(ctx: &mut Logger, decl_asset: DeclAsset, declared: &[Asset]) -> Compiled<Asset> {
    let key = match &decl_asset {
        DeclAsset::Material(key) => key,
        DeclAsset::Animation(key) => key,
    };
    let asset_type = match &decl_asset {
        DeclAsset::Material(_) => AssetType::Material,
        DeclAsset::Animation(_) => AssetType::Animation,
    };

    if let Some(defined) = declared.iter().find(|a| a.key == *key) {
        if defined.asset_type != asset_type {
            ctx.log(Log::IncompatibleAssetDeclaration(key.to_string()));
        }
        return failure();
    }

    success(Asset {
        asset_type,
        key: key.to_string(),
    })
}
