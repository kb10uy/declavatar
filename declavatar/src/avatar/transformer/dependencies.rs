use crate::avatar::data::{AnimationGroup, Asset, Parameter};

pub struct CompiledSources {
    parameters: Vec<Parameter>,
    assets: Vec<Asset>,
}

impl CompiledSources {
    pub fn new(parameters: Vec<Parameter>, assets: Vec<Asset>) -> CompiledSources {
        CompiledSources { parameters, assets }
    }
}

pub struct CompiledAnimations {
    sources: CompiledSources,
    animations: Vec<AnimationGroup>,
}

impl CompiledAnimations {
    pub fn new(
        dependencies: CompiledSources,
        animations: Vec<AnimationGroup>,
    ) -> CompiledAnimations {
        CompiledAnimations {
            sources: dependencies,
            animations,
        }
    }

    pub fn deconstruct(self) -> (Vec<Parameter>, Vec<Asset>, Vec<AnimationGroup>) {
        (
            self.sources.parameters,
            self.sources.assets,
            self.animations,
        )
    }
}
