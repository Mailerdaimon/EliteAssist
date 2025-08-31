use crate::gui::components::{details, sub_header};
use crate::gui::{pane, Message};
use crate::state::State;
use crate::theme::style;
use iced::widget::{column, scrollable};

pub struct Loadout;

impl pane::Type for Loadout {

    fn title(&self) -> &'static str { "Loadout" }
    
    fn render<'a>(&self, state: &'a State) -> iced::Element<'a, Message> {
        column![
            scrollable(column![
                sub_header("Suit"),
                details("Name", state.suit_loadout.suit_name),
                details("Class", state.suit_loadout.class.to_string()),
                details("Loadout", state.suit_loadout.loadout_name.as_ref()),
                column(
                    state
                        .suit_loadout
                        .suit_mods
                        .iter()
                        .map(|mod_name| { details("Modification", *mod_name).into() })
                )
                .padding(8),
                sub_header("Weapons"),
                column(state.suit_loadout.modules.iter().map(|module| {
                    column![
                        details(&module.slot_name, module.module_name.as_ref()),
                        details("Class", module.class.to_string()),
                        column(
                            module
                                .weapon_mods
                                .iter()
                                .map(|mod_name| { details("Modification", *mod_name).into() })
                        )
                        .padding([0, 16])
                    ]
                    .into()
                }))
                .padding(8)
            ])
            .style(style::scrollable)
        ]
        .into()
    }
}
