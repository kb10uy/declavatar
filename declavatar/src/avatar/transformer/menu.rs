use crate::{
    avatar::{
        data::{MenuBoolean, MenuGroup, MenuItem},
        transformer::{
            dependencies::{CompiledAnimations, CompiledSources},
            failure, success, Compiled, Context, LogKind,
        },
    },
    decl::data::{
        BooleanControlTarget as DeclBooleanControlTarget, Menu as DeclMenu,
        MenuElement as DeclMenuElement, PuppetAxes as DeclPuppetAxes,
    },
};

pub fn compile_menu(
    ctx: &mut Context,
    animations: &CompiledAnimations,
    decl_menu_blocks: Vec<DeclMenu>,
) -> Compiled<MenuGroup> {
    let menu_elements = decl_menu_blocks
        .into_iter()
        .flat_map(|ab| ab.elements)
        .collect();
    compile_menu_group(ctx, animations, "", menu_elements)
}

fn compile_menu_group(
    ctx: &mut Context,
    animations: &CompiledAnimations,
    name: impl Into<String>,
    decl_menu_elements: Vec<DeclMenuElement>,
) -> Compiled<MenuGroup> {
    let name = name.into();
    let mut items = vec![];

    for menu_element in decl_menu_elements {
        let Some(menu_item) = (match menu_element {
            DeclMenuElement::SubMenu(sm) => {
                compile_menu_group(ctx, animations, sm.name, sm.elements).map(MenuItem::SubMenu)
            }
            DeclMenuElement::Boolean(bc) => {
                let inner = compile_boolean(ctx, animations, bc.name, bc.target);
                if bc.toggle {
                    inner.map(MenuItem::Toggle)
                } else {
                    inner.map(MenuItem::Button)
                }
            }
            DeclMenuElement::Puppet(p) => compile_puppet(ctx, animations, p.name, p.axes),
        }) else {
            continue;
        };
        items.push(menu_item);
    }

    success(MenuGroup { name, items, id: 0 })
}

fn compile_boolean(
    ctx: &mut Context,
    animations: &CompiledAnimations,
    name: impl Into<String>,
    target: DeclBooleanControlTarget,
) -> Compiled<MenuBoolean> {
    failure()
}

fn compile_puppet(
    ctx: &mut Context,
    animations: &CompiledAnimations,
    name: impl Into<String>,
    axes: DeclPuppetAxes,
) -> Compiled<MenuItem> {
    failure()
}
