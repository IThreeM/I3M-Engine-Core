//! Property editor for [ImmutableString] properties based upon a [TextBox](crate::text_box::TextBox) widget.
use i3m_core::sstorage::ImmutableString;

use crate::{
    core::algebra::Vector2,
    formatted_text::WrapMode,
    inspector::{
        editors::{
            PropertyEditorBuildContext, PropertyEditorDefinition, PropertyEditorInstance,
            PropertyEditorMessageContext, PropertyEditorTranslationContext,
        },
        FieldKind, InspectorError, PropertyChanged,
    },
    message::{MessageDirection, UiMessage},
    text::TextMessage,
    text_box::{TextBoxBuilder, TextCommitMode},
    widget::WidgetBuilder,
    Thickness, VerticalAlignment,
};
use std::any::TypeId;

/// Property editor for [ImmutableString] properties based upon a [TextBox](crate::text_box::TextBox) widget.
#[derive(Debug)]
pub struct ImmutableStringPropertyEditorDefinition;

impl PropertyEditorDefinition for ImmutableStringPropertyEditorDefinition {
    fn value_type_id(&self) -> TypeId {
        TypeId::of::<ImmutableString>()
    }

    fn create_instance(
        &self,
        ctx: PropertyEditorBuildContext,
    ) -> Result<PropertyEditorInstance, InspectorError> {
        let value = ctx.property_info.cast_value::<ImmutableString>()?;
        Ok(PropertyEditorInstance::Simple {
            editor: TextBoxBuilder::new(
                WidgetBuilder::new()
                    .with_min_size(Vector2::new(0.0, 17.0))
                    .with_margin(Thickness::uniform(1.0)),
            )
            .with_wrap(WrapMode::Word)
            .with_text_commit_mode(TextCommitMode::Changed)
            .with_text(value)
            .with_vertical_text_alignment(VerticalAlignment::Center)
            .build(ctx.build_context),
        })
    }

    fn create_message(
        &self,
        ctx: PropertyEditorMessageContext,
    ) -> Result<Option<UiMessage>, InspectorError> {
        let value = ctx.property_info.cast_value::<ImmutableString>()?;
        Ok(Some(TextMessage::text(
            ctx.instance,
            MessageDirection::ToWidget,
            value.to_mutable(),
        )))
    }

    fn translate_message(&self, ctx: PropertyEditorTranslationContext) -> Option<PropertyChanged> {
        if ctx.message.direction() == MessageDirection::FromWidget {
            if let Some(TextMessage::Text(value)) = ctx.message.data::<TextMessage>() {
                return Some(PropertyChanged {
                    owner_type_id: ctx.owner_type_id,
                    name: ctx.name.to_string(),
                    value: FieldKind::object(ImmutableString::new(value)),
                });
            }
        }
        None
    }
}
