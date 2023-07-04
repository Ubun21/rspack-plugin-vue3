use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
pub enum ErrorCodes {
  AbruptClosingOfEmptyComment,
  CdataInHtmlContent,
  DuplicateAttribute,
  EndTagWithAttributes,
  EndTagWithTrailingSolidus,
  EofBeforeTagName,
  EofInCdata,
  EofInComment,
  EofInScriptHtmlCommentLikeText,
  EofInTag,
  IncorrectlyClosedComment,
  IncorrectlyOpenedComment,
  InvalidFirstCharacterOfTagName,
  MissingAttributeValue,
  MissingEndTagName,
  MissingWhitespaceBetweenAttributes,
  NestedComment,
  UnexpectedCharacterInAttributeName,
  UnexpectedCharacterInUnquotedAttributeValue,
  UnexpectedEqualsSignBeforeAttributeName,
  UnexpectedNullCharacter,
  UnexpectedQuestionMarkInsteadOfTagName,
  UnexpectedSolidusInTag,

  // Vue-specific parse errors
  XInvalidEndTag,
  XMissingEndTag,
  XMissingInterpolationEnd,
  XMissingDirectiveName,
  XMissingDynamicDirectiveArgumentEnd,

  // transform errors
  XVIfNoExpression,
  XVIfSameKey,
  XVElseNoAdjacentIf,
  XVForNoExpression,
  XVForMalformedExpression,
  XVForTemplateKeyPlacement,
  XVBindNoExpression,
  XVOnNoExpression,
  XVSlotUnexpectedDirectiveOnSlotOutlet,
  XVSlotMixedSlotUsage,
  XVSlotDuplicateSlotNames,
  XVSlotExtraneousDefaultSlotChildren,
  XVSlotMisplaced,
  XVModelNoExpression,
  XVModelMalformedExpression,
  XVModelOnScopeVariable,
  XVModelOnProps,
  XInvalidExpression,
  XKeepAliveInvalidChildren,

  // generic errors
  XPrefixIdNotSupported,
  XModuleModeNotSupported,
  XCacheHandlerNotSupported,
  XScopeIdNotSupported,

  // deprecations
  DeprecationVnodeHooks,
  DeprecationVIs,

  // Special value for higher-order compilers to pick up the last code
  // to avoid collision of error codes. This should always be kept as the last
  // item.
  ExtendPoint
}

lazy_static! {
  pub static ref ERROR_MESSAGES: HashMap<ErrorCodes, &'static str> = {
      let mut map = HashMap::new();
      map.insert(ErrorCodes::AbruptClosingOfEmptyComment, "Illegal comment.");
      map.insert(ErrorCodes::CdataInHtmlContent, "CDATA section is allowed only in XML context.");
      map.insert(ErrorCodes::DuplicateAttribute, "Duplicate attribute.");
      map.insert(ErrorCodes::EndTagWithAttributes, "End tag cannot have attributes.");
      map.insert(ErrorCodes::EndTagWithTrailingSolidus, "Illegal '/' in tags.");
      map.insert(ErrorCodes::EofBeforeTagName, "Unexpected EOF in tag.");
      map.insert(ErrorCodes::EofInCdata, "Unexpected EOF in CDATA section.");
      map.insert(ErrorCodes::EofInComment, "Unexpected EOF in comment.");
      map.insert(ErrorCodes::EofInScriptHtmlCommentLikeText, "Unexpected EOF in script.");
      map.insert(ErrorCodes::EofInTag, "Unexpected EOF in tag.");
      map.insert(ErrorCodes::IncorrectlyClosedComment, "Incorrectly closed comment.");
      map.insert(ErrorCodes::IncorrectlyOpenedComment, "Incorrectly opened comment.");
      map.insert(ErrorCodes::InvalidFirstCharacterOfTagName, "Illegal tag name. Use '&lt;' to print '<'.");
      map.insert(ErrorCodes::MissingAttributeValue, "Attribute value was expected.");
      map.insert(ErrorCodes::MissingEndTagName, "End tag name was expected.");
      map.insert(ErrorCodes::MissingWhitespaceBetweenAttributes, "Whitespace was expected.");
      map.insert(ErrorCodes::NestedComment, "Unexpected '<!--' in comment.");
      map.insert(ErrorCodes::UnexpectedCharacterInAttributeName, "Attribute name cannot contain U+0022 (\"), U+0027 ('), and U+003C (<).");
      map.insert(ErrorCodes::UnexpectedCharacterInUnquotedAttributeValue, "Unquoted attribute value cannot contain U+0022 (\"), U+0027 ('), U+003C (<), U+003D (=), and U+0060 (`).");
      map.insert(ErrorCodes::UnexpectedEqualsSignBeforeAttributeName, "Attribute name cannot start with '='.");
      map.insert(ErrorCodes::UnexpectedNullCharacter, "NULL character is not allowed in XML documents.");
      map.insert(ErrorCodes::UnexpectedQuestionMarkInsteadOfTagName, "'<?' is allowed only in XML context.");
      map.insert(ErrorCodes::UnexpectedSolidusInTag, "Illegal '/' in tags.");
      map.insert(ErrorCodes::XInvalidEndTag, "Invalid end tag.");
      map.insert(ErrorCodes::XMissingEndTag, "End tag was not found.");
      map.insert(ErrorCodes::XMissingInterpolationEnd, "Interpolation end sign was not found.");
      map.insert(ErrorCodes::XMissingDirectiveName, "Directive name was not found.");
      map.insert(ErrorCodes::XMissingDynamicDirectiveArgumentEnd, "End bracket for dynamic directive argument was not found. Note that dynamic directive argument cannot contain spaces.");
      map.insert(ErrorCodes::XVIfNoExpression, "v-if/v-else-if is missing expression.");
      map.insert(ErrorCodes::XVIfSameKey, "v-if/else branches must use unique keys.");
      map.insert(ErrorCodes::XVElseNoAdjacentIf, "v-else/v-else-if has no adjacent v-if.");
      map.insert(ErrorCodes::XVForNoExpression, "v-for is missing expression.");
      map.insert(ErrorCodes::XVForMalformedExpression, "v-for has invalid expression.");
      map.insert(ErrorCodes::XVForTemplateKeyPlacement, "v-for template key should be placed on <template> tag.");
      map.insert(ErrorCodes::XVBindNoExpression, "v-bind is missing expression.");
      map.insert(ErrorCodes::XVOnNoExpression, "v-on is missing expression.");
      map.insert(ErrorCodes::XVSlotUnexpectedDirectiveOnSlotOutlet, "Unexpected custom directive on <slot> outlet.");
      map.insert(ErrorCodes::XVSlotMixedSlotUsage, "Mixed v-slot usage on both the component and nested <template>s. When there are multiple named slots, all slots should use <template> syntax to avoid scope ambiguity.");
      map.insert(ErrorCodes::XVSlotDuplicateSlotNames, "Duplicate slot names found. ");
      map.insert(ErrorCodes::XVSlotExtraneousDefaultSlotChildren, "Extraneous children found when component already has explicitly named default slot. ");
      map.insert(ErrorCodes::XVSlotMisplaced, "v-slot can only be used on components or <template> tags.");
      map.insert(ErrorCodes::XVModelNoExpression, "v-model is missing expression.");
      map.insert(ErrorCodes::XVModelMalformedExpression, "v-model value must be a valid JavaScript member expression.");
      map.insert(ErrorCodes::XVModelOnScopeVariable, "v-model cannot be used on v-for or v-slot scope variables because they are not writable.");
      map.insert(ErrorCodes::XVModelOnProps, "v-model cannot be used on an aliased v-bind in Vue < 2.6.0.");
      map.insert(ErrorCodes::XInvalidExpression, "Error parsing JavaScript expression: ");
      map.insert(ErrorCodes::XKeepAliveInvalidChildren, "<KeepAlive> expects exactly one child component.");
      map.insert(ErrorCodes::XPrefixIdNotSupported, "Prefix id feature is not supported in this build of compiler.");
      map.insert(ErrorCodes::XModuleModeNotSupported, "ES module mode is not supported in this build of compiler.");
      map.insert(ErrorCodes::XCacheHandlerNotSupported, "Cache handler is not supported in this build of compiler.");
      map.insert(ErrorCodes::XScopeIdNotSupported, "scopeId option is not supported in this build of compiler.");
      map.insert(ErrorCodes::DeprecationVnodeHooks, "vnode hooks (e.g. v-once) are deprecated. Use <template v-slot> instead");
      map.insert(ErrorCodes::DeprecationVIs, "`v-is` on <template> or custom components is deprecated. Use the `is` special attribute instead.");
      map.insert(ErrorCodes::ExtendPoint, "Internal ExtendPoint");
      map
  };
}
