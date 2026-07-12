[bevy](../index.html)::[a11y](index.html)

# Struct AccessibilityNode 

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#227)

```rust
pub struct AccessibilityNode(pub Node);
```

Represents an entity to `AccessKit` through an [`accesskit::Node`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html "struct accesskit::Node").

Platform-specific accessibility APIs utilize `AccessKit` nodes in their accessibility frameworks. So, this component acts as a translation between “Bevy entity” and “platform-agnostic accessibility element”.

### Organization in the `AccessKit` Accessibility Tree

`AccessKit` allows users to form a “tree of nodes” providing accessibility information. That tree is **not** Bevy’s ECS!

To explain, let’s say this component is added to an entity, `E`.

#### Parent and Child

If `E` has a parent, `P`, and `P` also has this `AccessibilityNode` component, then `E`’s `AccessKit` node will be a child of `P`’s `AccessKit` node.

Resulting `AccessKit` tree:

*   P
    *   E

In other words, parent-child relationships are maintained, but only if both have this component.

#### On the Window

If `E` doesn’t have a parent, or if the immediate parent doesn’t have an `AccessibilityNode`, its `AccessKit` node will be an immediate child of the primary window.

Resulting `AccessKit` tree:

*   Primary window
    *   E

When there’s no `AccessKit`\-compatible parent, the child lacks hierarchical information in `AccessKit`. As such, it is placed directly under the primary window on the `AccessKit` tree.

This behavior may or may not be intended, so please utilize `AccessibilityNode`s with care.

## Tuple Fields

`0: [Node](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html "struct accesskit::Node")`

A representation of this component’s entity to `AccessKit`.

Note that, with its parent struct acting as just a newtype, users are intended to directly update this field.

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [Node](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html "struct accesskit::Node")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1747)

#### pub fn [role](#method.role)(&self) -> [Role](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Role.html "enum accesskit::Role")

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1751)

#### pub fn [set\_role](#method.set_role)(&mut self, value: [Role](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Role.html "enum accesskit::Role"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1756)

#### pub fn [supports\_action](#method.supports_action)(&self, action: [Action](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Action.html "enum accesskit::Action")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1760)

#### pub fn [add\_action](#method.add_action)(&mut self, action: [Action](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Action.html "enum accesskit::Action"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1764)

#### pub fn [remove\_action](#method.remove_action)(&mut self, action: [Action](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Action.html "enum accesskit::Action"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1768)

#### pub fn [clear\_actions](#method.clear_actions)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1775)

#### pub fn [child\_supports\_action](#method.child_supports_action)(&self, action: [Action](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Action.html "enum accesskit::Action")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Return whether the specified action is in the set supported on this node’s direct children in the filtered tree.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1781)

#### pub fn [add\_child\_action](#method.add_child_action)(&mut self, action: [Action](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Action.html "enum accesskit::Action"))

Add the specified action to the set supported on this node’s direct children in the filtered tree.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1787)

#### pub fn [remove\_child\_action](#method.remove_child_action)(&mut self, action: [Action](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Action.html "enum accesskit::Action"))

Remove the specified action from the set supported on this node’s direct children in the filtered tree.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1793)

#### pub fn [clear\_child\_actions](#method.clear_child_actions)(&mut self)

Clear the set of actions supported on this node’s direct children in the filtered tree.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_hidden](#method.is_hidden)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Exclude this node and its descendants from the tree presented to assistive technologies, and from hit testing.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_hidden](#method.set_hidden)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_hidden](#method.clear_hidden)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_multiselectable](#method.is_multiselectable)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_multiselectable](#method.set_multiselectable)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_multiselectable](#method.clear_multiselectable)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_required](#method.is_required)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_required](#method.set_required)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_required](#method.clear_required)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_visited](#method.is_visited)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_visited](#method.set_visited)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_visited](#method.clear_visited)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_busy](#method.is_busy)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_busy](#method.set_busy)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_busy](#method.clear_busy)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_live\_atomic](#method.is_live_atomic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_live\_atomic](#method.set_live_atomic)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_live\_atomic](#method.clear_live_atomic)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_modal](#method.is_modal)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

If a dialog box is marked as explicitly modal.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_modal](#method.set_modal)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_modal](#method.clear_modal)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_touch\_transparent](#method.is_touch_transparent)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This element allows touches to be passed through when a screen reader is in touch exploration mode, e.g. a virtual keyboard normally behaves this way.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_touch\_transparent](#method.set_touch_transparent)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_touch\_transparent](#method.clear_touch_transparent)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_read\_only](#method.is_read_only)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Use for a text widget that allows focus/selection but not input.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_read\_only](#method.set_read_only)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_read\_only](#method.clear_read_only)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_disabled](#method.is_disabled)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Use for a control or group of controls that disallows input.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_disabled](#method.set_disabled)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_disabled](#method.clear_disabled)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_italic](#method.is_italic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_italic](#method.set_italic)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_italic](#method.clear_italic)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clips\_children](#method.clips_children)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates that this node clips its children, i.e. may have `overflow: hidden` or clip children by default.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_clips\_children](#method.set_clips_children)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_clips\_children](#method.clear_clips_children)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_line\_breaking\_object](#method.is_line_breaking_object)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether this node causes a hard line-break (e.g. block level elements, or `<br>`).

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_is\_line\_breaking\_object](#method.set_is_line_breaking_object)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_is\_line\_breaking\_object](#method.clear_is_line_breaking_object)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_page\_breaking\_object](#method.is_page_breaking_object)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether this node causes a page break.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_is\_page\_breaking\_object](#method.set_is_page_breaking_object)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_is\_page\_breaking\_object](#method.clear_is_page_breaking_object)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_spelling\_error](#method.is_spelling_error)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_is\_spelling\_error](#method.set_is_spelling_error)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_is\_spelling\_error](#method.clear_is_spelling_error)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_grammar\_error](#method.is_grammar_error)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_is\_grammar\_error](#method.set_is_grammar_error)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_is\_grammar\_error](#method.clear_is_grammar_error)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_search\_match](#method.is_search_match)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_is\_search\_match](#method.set_is_search_match)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_is\_search\_match](#method.clear_is_search_match)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [is\_suggestion](#method.is_suggestion)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [set\_is\_suggestion](#method.set_is_suggestion)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1798-1830)

#### pub fn [clear\_is\_suggestion](#method.clear_is_suggestion)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [children](#method.children)(&self) -> &\[[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\]

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [set\_children](#method.set_children)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [clear\_children](#method.clear_children)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [push\_child](#method.push_child)(&mut self, item: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [controls](#method.controls)(&self) -> &\[[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\]

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [set\_controls](#method.set_controls)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [clear\_controls](#method.clear_controls)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [push\_controlled](#method.push_controlled)(&mut self, item: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [details](#method.details)(&self) -> &\[[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\]

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [set\_details](#method.set_details)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [clear\_details](#method.clear_details)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [push\_detail](#method.push_detail)(&mut self, item: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [described\_by](#method.described_by)(&self) -> &\[[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\]

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [set\_described\_by](#method.set_described_by)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [clear\_described\_by](#method.clear_described_by)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [push\_described\_by](#method.push_described_by)(&mut self, item: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [flow\_to](#method.flow_to)(&self) -> &\[[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\]

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [set\_flow\_to](#method.set_flow_to)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [clear\_flow\_to](#method.clear_flow_to)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [push\_flow\_to](#method.push_flow_to)(&mut self, item: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [labelled\_by](#method.labelled_by)(&self) -> &\[[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\]

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [set\_labelled\_by](#method.set_labelled_by)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [clear\_labelled\_by](#method.clear_labelled_by)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [push\_labelled\_by](#method.push_labelled_by)(&mut self, item: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [owns](#method.owns)(&self) -> &\[[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\]

As with the `aria-owns` property in ARIA, this property should be set only if the nodes referenced in the property are not descendants of the owning node in the AccessKit tree. In the common case, where the owned nodes are direct children or indirect descendants, this property is unnecessary.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [set\_owns](#method.set_owns)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [clear\_owns](#method.clear_owns)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [push\_owned](#method.push_owned)(&mut self, item: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [radio\_group](#method.radio_group)(&self) -> &\[[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\]

On radio buttons this should be set to a list of all of the buttons in the same group as this one, including this radio button itself.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [set\_radio\_group](#method.set_radio_group)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [clear\_radio\_group](#method.clear_radio_group)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1880-1896)

#### pub fn [push\_to\_radio\_group](#method.push_to_radio_group)(&mut self, item: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [active\_descendant](#method.active_descendant)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>

For a composite widget such as a listbox, tree, or grid, identifies the currently active descendant. Used when focus remains on the container while the active item changes.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [set\_active\_descendant](#method.set_active_descendant)(&mut self, value: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [clear\_active\_descendant](#method.clear_active_descendant)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [error\_message](#method.error_message)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [set\_error\_message](#method.set_error_message)(&mut self, value: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [clear\_error\_message](#method.clear_error_message)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [in\_page\_link\_target](#method.in_page_link_target)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [set\_in\_page\_link\_target](#method.set_in_page_link_target)(&mut self, value: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [clear\_in\_page\_link\_target](#method.clear_in_page_link_target)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [member\_of](#method.member_of)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [set\_member\_of](#method.set_member_of)(&mut self, value: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [clear\_member\_of](#method.clear_member_of)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [next\_on\_line](#method.next_on_line)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [set\_next\_on\_line](#method.set_next_on_line)(&mut self, value: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [clear\_next\_on\_line](#method.clear_next_on_line)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [previous\_on\_line](#method.previous_on_line)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [set\_previous\_on\_line](#method.set_previous_on_line)(&mut self, value: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [clear\_previous\_on\_line](#method.clear_previous_on_line)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [popup\_for](#method.popup_for)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [set\_popup\_for](#method.set_popup_for)(&mut self, value: [NodeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.NodeId.html "struct accesskit::NodeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1898-1909)

#### pub fn [clear\_popup\_for](#method.clear_popup_for)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [label](#method.label)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

The label of a control that can have a label. If the label is specified via the [`Node::labelled_by`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.labelled_by "method accesskit::Node::labelled_by") relation, this doesn’t need to be set. Note that the text content of a node with the [`Role::Label`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Role.html#variant.Label "variant accesskit::Role::Label") role should be provided via [`Node::value`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.value "method accesskit::Node::value"), not this property.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_label](#method.set_label)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_label](#method.clear_label)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [description](#method.description)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_description](#method.set_description)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_description](#method.clear_description)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [value](#method.value)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_value](#method.set_value)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_value](#method.clear_value)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [access\_key](#method.access_key)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

A single character, usually part of this node’s name, that can be pressed, possibly along with a platform-specific modifier, to perform this node’s default action. For menu items, the access key is only active while the menu is active, in contrast with [`keyboard_shortcut`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.keyboard_shortcut "method accesskit::Node::keyboard_shortcut"); a single menu item may in fact have both properties.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_access\_key](#method.set_access_key)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_access\_key](#method.clear_access_key)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [author\_id](#method.author_id)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

A way for application authors to identify this node for automated testing purpose. The value must be unique among this node’s siblings.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_author\_id](#method.set_author_id)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_author\_id](#method.clear_author_id)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [class\_name](#method.class_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_class\_name](#method.set_class_name)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_class\_name](#method.clear_class_name)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [font\_family](#method.font_family)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Only present when different from parent.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_font\_family](#method.set_font_family)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_font\_family](#method.clear_font_family)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [html\_tag](#method.html_tag)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_html\_tag](#method.set_html_tag)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_html\_tag](#method.clear_html_tag)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [inner\_html](#method.inner_html)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Inner HTML of an element. Only used for a top-level math element, to support third-party math accessibility products that parse MathML.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_inner\_html](#method.set_inner_html)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_inner\_html](#method.clear_inner_html)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [keyboard\_shortcut](#method.keyboard_shortcut)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

A keystroke or sequence of keystrokes, complete with any required modifiers(s), that will perform this node’s default action. The value of this property should be in a human-friendly format.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_keyboard\_shortcut](#method.set_keyboard_shortcut)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_keyboard\_shortcut](#method.clear_keyboard_shortcut)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [language](#method.language)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

An [IETF language tag](https://www.rfc-editor.org/info/bcp47). Only present when different from parent.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_language](#method.set_language)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_language](#method.clear_language)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [placeholder](#method.placeholder)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

If a text input has placeholder text, it should be exposed through this property rather than [`label`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.label "method accesskit::Node::label").

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_placeholder](#method.set_placeholder)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_placeholder](#method.clear_placeholder)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [role\_description](#method.role_description)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

An optional string that may override an assistive technology’s description of the node’s role. Only provide this for custom control types. The value of this property should be in a human-friendly, localized format.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_role\_description](#method.set_role_description)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_role\_description](#method.clear_role_description)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [state\_description](#method.state_description)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

An optional string that may override an assistive technology’s description of the node’s state, replacing default strings such as “checked” or “selected”. Note that most platform accessibility APIs and assistive technologies do not support this feature.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_state\_description](#method.set_state_description)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_state\_description](#method.clear_state_description)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [tooltip](#method.tooltip)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

If a node’s only accessible name comes from a tooltip, it should be exposed through this property rather than [`label`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.label "method accesskit::Node::label").

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_tooltip](#method.set_tooltip)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_tooltip](#method.clear_tooltip)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [url](#method.url)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_url](#method.set_url)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_url](#method.clear_url)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [row\_index\_text](#method.row_index_text)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_row\_index\_text](#method.set_row_index_text)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_row\_index\_text](#method.clear_row_index_text)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [column\_index\_text](#method.column_index_text)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_column\_index\_text](#method.set_column_index_text)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_column\_index\_text](#method.clear_column_index_text)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [braille\_label](#method.braille_label)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_braille\_label](#method.set_braille_label)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_braille\_label](#method.clear_braille_label)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [braille\_role\_description](#method.braille_role_description)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [set\_braille\_role\_description](#method.set_braille_role_description)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1911-1968)

#### pub fn [clear\_braille\_role\_description](#method.clear_braille_role_description)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [scroll\_x](#method.scroll_x)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [set\_scroll\_x](#method.set_scroll_x)(&mut self, value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [clear\_scroll\_x](#method.clear_scroll_x)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [scroll\_x\_min](#method.scroll_x_min)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [set\_scroll\_x\_min](#method.set_scroll_x_min)(&mut self, value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [clear\_scroll\_x\_min](#method.clear_scroll_x_min)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [scroll\_x\_max](#method.scroll_x_max)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [set\_scroll\_x\_max](#method.set_scroll_x_max)(&mut self, value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [clear\_scroll\_x\_max](#method.clear_scroll_x_max)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [scroll\_y](#method.scroll_y)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [set\_scroll\_y](#method.set_scroll_y)(&mut self, value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [clear\_scroll\_y](#method.clear_scroll_y)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [scroll\_y\_min](#method.scroll_y_min)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [set\_scroll\_y\_min](#method.set_scroll_y_min)(&mut self, value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [clear\_scroll\_y\_min](#method.clear_scroll_y_min)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [scroll\_y\_max](#method.scroll_y_max)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [set\_scroll\_y\_max](#method.set_scroll_y_max)(&mut self, value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [clear\_scroll\_y\_max](#method.clear_scroll_y_max)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [numeric\_value](#method.numeric_value)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [set\_numeric\_value](#method.set_numeric_value)(&mut self, value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [clear\_numeric\_value](#method.clear_numeric_value)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [min\_numeric\_value](#method.min_numeric_value)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [set\_min\_numeric\_value](#method.set_min_numeric_value)(&mut self, value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [clear\_min\_numeric\_value](#method.clear_min_numeric_value)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [max\_numeric\_value](#method.max_numeric_value)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [set\_max\_numeric\_value](#method.set_max_numeric_value)(&mut self, value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [clear\_max\_numeric\_value](#method.clear_max_numeric_value)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [numeric\_value\_step](#method.numeric_value_step)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [set\_numeric\_value\_step](#method.set_numeric_value_step)(&mut self, value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [clear\_numeric\_value\_step](#method.clear_numeric_value_step)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [numeric\_value\_jump](#method.numeric_value_jump)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [set\_numeric\_value\_jump](#method.set_numeric_value_jump)(&mut self, value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1970-1982)

#### pub fn [clear\_numeric\_value\_jump](#method.clear_numeric_value_jump)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1984-1990)

#### pub fn [font\_size](#method.font_size)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>

Font size is in pixels.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1984-1990)

#### pub fn [set\_font\_size](#method.set_font_size)(&mut self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1984-1990)

#### pub fn [clear\_font\_size](#method.clear_font_size)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1984-1990)

#### pub fn [font\_weight](#method.font_weight)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>

Font weight can take on any arbitrary numeric value. Increments of 100 in range `[0, 900]` represent keywords such as light, normal, bold, etc.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1984-1990)

#### pub fn [set\_font\_weight](#method.set_font_weight)(&mut self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1984-1990)

#### pub fn [clear\_font\_weight](#method.clear_font_weight)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [row\_count](#method.row_count)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [set\_row\_count](#method.set_row_count)(&mut self, value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [clear\_row\_count](#method.clear_row_count)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [column\_count](#method.column_count)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [set\_column\_count](#method.set_column_count)(&mut self, value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [clear\_column\_count](#method.clear_column_count)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [row\_index](#method.row_index)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [set\_row\_index](#method.set_row_index)(&mut self, value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [clear\_row\_index](#method.clear_row_index)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [column\_index](#method.column_index)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [set\_column\_index](#method.set_column_index)(&mut self, value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [clear\_column\_index](#method.clear_column_index)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [row\_span](#method.row_span)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [set\_row\_span](#method.set_row_span)(&mut self, value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [clear\_row\_span](#method.clear_row_span)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [column\_span](#method.column_span)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [set\_column\_span](#method.set_column_span)(&mut self, value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [clear\_column\_span](#method.clear_column_span)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [level](#method.level)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [set\_level](#method.set_level)(&mut self, value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [clear\_level](#method.clear_level)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [size\_of\_set](#method.size_of_set)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

For containers like [`Role::ListBox`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Role.html#variant.ListBox "variant accesskit::Role::ListBox"), specifies the total number of items.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [set\_size\_of\_set](#method.set_size_of_set)(&mut self, value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [clear\_size\_of\_set](#method.clear_size_of_set)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [position\_in\_set](#method.position_in_set)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

For items like [`Role::ListBoxOption`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Role.html#variant.ListBoxOption "variant accesskit::Role::ListBoxOption"), specifies their index in the item list. This may not exceed the value of [`size_of_set`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.size_of_set "method accesskit::Node::size_of_set") as set on the container.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [set\_position\_in\_set](#method.set_position_in_set)(&mut self, value: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#1992-2007)

#### pub fn [clear\_position\_in\_set](#method.clear_position_in_set)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2009-2016)

#### pub fn [color\_value](#method.color_value)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Color](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Color.html "struct accesskit::Color")\>

For [`Role::ColorWell`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Role.html#variant.ColorWell "variant accesskit::Role::ColorWell"), specifies the selected color.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2009-2016)

#### pub fn [set\_color\_value](#method.set_color_value)(&mut self, value: [Color](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Color.html "struct accesskit::Color"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2009-2016)

#### pub fn [clear\_color\_value](#method.clear_color_value)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2009-2016)

#### pub fn [background\_color](#method.background_color)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Color](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Color.html "struct accesskit::Color")\>

Background color.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2009-2016)

#### pub fn [set\_background\_color](#method.set_background_color)(&mut self, value: [Color](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Color.html "struct accesskit::Color"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2009-2016)

#### pub fn [clear\_background\_color](#method.clear_background_color)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2009-2016)

#### pub fn [foreground\_color](#method.foreground_color)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Color](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Color.html "struct accesskit::Color")\>

Foreground color.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2009-2016)

#### pub fn [set\_foreground\_color](#method.set_foreground_color)(&mut self, value: [Color](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Color.html "struct accesskit::Color"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2009-2016)

#### pub fn [clear\_foreground\_color](#method.clear_foreground_color)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2018-2022)

#### pub fn [overline](#method.overline)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TextDecoration](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.TextDecoration.html "struct accesskit::TextDecoration")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2018-2022)

#### pub fn [set\_overline](#method.set_overline)(&mut self, value: [TextDecoration](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.TextDecoration.html "struct accesskit::TextDecoration"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2018-2022)

#### pub fn [clear\_overline](#method.clear_overline)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2018-2022)

#### pub fn [strikethrough](#method.strikethrough)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TextDecoration](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.TextDecoration.html "struct accesskit::TextDecoration")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2018-2022)

#### pub fn [set\_strikethrough](#method.set_strikethrough)(&mut self, value: [TextDecoration](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.TextDecoration.html "struct accesskit::TextDecoration"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2018-2022)

#### pub fn [clear\_strikethrough](#method.clear_strikethrough)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2018-2022)

#### pub fn [underline](#method.underline)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TextDecoration](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.TextDecoration.html "struct accesskit::TextDecoration")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2018-2022)

#### pub fn [set\_underline](#method.set_underline)(&mut self, value: [TextDecoration](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.TextDecoration.html "struct accesskit::TextDecoration"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2018-2022)

#### pub fn [clear\_underline](#method.clear_underline)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2024-2075)

#### pub fn [character\_lengths](#method.character_lengths)(&self) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\] [ⓘ](#)

For text runs, the length (non-inclusive) of each character in UTF-8 code units (bytes). The sum of these lengths must equal the length of [`value`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.value "method accesskit::Node::value"), also in bytes.

A character is defined as the smallest unit of text that can be selected. This isn’t necessarily a single Unicode scalar value (code point). This is why AccessKit can’t compute the lengths of the characters from the text itself; this information must be provided by the text editing implementation.

If this node is the last text run in a line that ends with a hard line break, that line break should be included at the end of this node’s value as either a CRLF or LF; in both cases, the line break should be counted as a single character for the sake of this slice. When the caret is at the end of such a line, the focus of the text selection should be on the line break, not after it.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2024-2075)

#### pub fn [set\_character\_lengths](#method.set_character_lengths)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2024-2075)

#### pub fn [clear\_character\_lengths](#method.clear_character_lengths)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2024-2075)

#### pub fn [word\_starts](#method.word_starts)(&self) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\] [ⓘ](#)

For text runs, the start index of each word in characters, as defined in [`character_lengths`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.character_lengths "method accesskit::Node::character_lengths"). This list must be sorted.

If this text run doesn’t contain the start of any words, but only the middle or end of a word, this list must be empty.

If this text run is the first in the document or the first in a paragraph (that is, the previous run ends with a newline character), then the first character of the run is implicitly the start of a word. In this case, beginning this list with `0` is permitted but not necessary.

The end of each word is the beginning of the next word; there are no characters that are not considered part of a word. Trailing whitespace is typically considered part of the word that precedes it, while a line’s leading whitespace is considered its own word. Whether punctuation is considered a separate word or part of the preceding word depends on the particular text editing implementation. Some editors may have their own definition of a word; for example, in an IDE, words may correspond to programming language tokens.

Not all assistive technologies require information about word boundaries, and not all platform accessibility APIs even expose this information, but for assistive technologies that do use this information, users will get unpredictable results if the word boundaries exposed by the accessibility tree don’t match the editor’s behavior. This is why AccessKit does not determine word boundaries itself.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2024-2075)

#### pub fn [set\_word\_starts](#method.set_word_starts)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2024-2075)

#### pub fn [clear\_word\_starts](#method.clear_word_starts)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2077-2115)

#### pub fn [character\_positions](#method.character_positions)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]>

For text runs, this is the position of each character within the node’s bounding box, in the direction given by [`text_direction`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.text_direction "method accesskit::Node::text_direction"), in the coordinate space of this node.

When present, the length of this slice should be the same as the length of [`character_lengths`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.character_lengths "method accesskit::Node::character_lengths"), including for lines that end with a hard line break. The position of such a line break should be the position where an end-of-paragraph marker would be rendered.

This property is optional. Without it, AccessKit can’t support some use cases, such as screen magnifiers that track the caret position or screen readers that display a highlight cursor. However, most text functionality still works without this information.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2077-2115)

#### pub fn [set\_character\_positions](#method.set_character_positions)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2077-2115)

#### pub fn [clear\_character\_positions](#method.clear_character_positions)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2077-2115)

#### pub fn [character\_widths](#method.character_widths)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]>

For text runs, this is the advance width of each character, in the direction given by [`text_direction`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.text_direction "method accesskit::Node::text_direction"), in the coordinate space of this node.

When present, the length of this slice should be the same as the length of [`character_lengths`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.character_lengths "method accesskit::Node::character_lengths"), including for lines that end with a hard line break. The width of such a line break should be non-zero if selecting the line break by itself results in a visible highlight (as in Microsoft Word), or zero if not (as in Windows Notepad).

This property is optional. Without it, AccessKit can’t support some use cases, such as screen magnifiers that track the caret position or screen readers that display a highlight cursor. However, most text functionality still works without this information.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2077-2115)

#### pub fn [set\_character\_widths](#method.set_character_widths)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2077-2115)

#### pub fn [clear\_character\_widths](#method.clear_character_widths)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2117-2134)

#### pub fn [is\_expanded](#method.is_expanded)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Whether this node is expanded, collapsed, or neither.

Setting this to `false` means the node is collapsed; omitting it means this state isn’t applicable.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2117-2134)

#### pub fn [set\_expanded](#method.set_expanded)(&mut self, value: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2117-2134)

#### pub fn [clear\_expanded](#method.clear_expanded)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2117-2134)

#### pub fn [is\_selected](#method.is_selected)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Indicates whether this node is selected or unselected.

The absence of this flag (as opposed to a `false` setting) means that the concept of “selected” doesn’t apply. When deciding whether to set the flag to false or omit it, consider whether it would be appropriate for a screen reader to announce “not selected”. The ambiguity of this flag in platform accessibility APIs has made extraneous “not selected” announcements a common annoyance.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2117-2134)

#### pub fn [set\_selected](#method.set_selected)(&mut self, value: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2117-2134)

#### pub fn [clear\_selected](#method.clear_selected)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [invalid](#method.invalid)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Invalid](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Invalid.html "enum accesskit::Invalid")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [set\_invalid](#method.set_invalid)(&mut self, value: [Invalid](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Invalid.html "enum accesskit::Invalid"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [clear\_invalid](#method.clear_invalid)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [toggled](#method.toggled)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Toggled](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Toggled.html "enum accesskit::Toggled")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [set\_toggled](#method.set_toggled)(&mut self, value: [Toggled](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Toggled.html "enum accesskit::Toggled"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [clear\_toggled](#method.clear_toggled)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [live](#method.live)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Live](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Live.html "enum accesskit::Live")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [set\_live](#method.set_live)(&mut self, value: [Live](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Live.html "enum accesskit::Live"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [clear\_live](#method.clear_live)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [text\_direction](#method.text_direction)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TextDirection](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.TextDirection.html "enum accesskit::TextDirection")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [set\_text\_direction](#method.set_text_direction)(&mut self, value: [TextDirection](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.TextDirection.html "enum accesskit::TextDirection"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [clear\_text\_direction](#method.clear_text_direction)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [orientation](#method.orientation)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Orientation](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Orientation.html "enum accesskit::Orientation")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [set\_orientation](#method.set_orientation)(&mut self, value: [Orientation](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.Orientation.html "enum accesskit::Orientation"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [clear\_orientation](#method.clear_orientation)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [sort\_direction](#method.sort_direction)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[SortDirection](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.SortDirection.html "enum accesskit::SortDirection")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [set\_sort\_direction](#method.set_sort_direction)(&mut self, value: [SortDirection](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.SortDirection.html "enum accesskit::SortDirection"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [clear\_sort\_direction](#method.clear_sort_direction)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [aria\_current](#method.aria_current)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[AriaCurrent](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.AriaCurrent.html "enum accesskit::AriaCurrent")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [set\_aria\_current](#method.set_aria_current)(&mut self, value: [AriaCurrent](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.AriaCurrent.html "enum accesskit::AriaCurrent"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [clear\_aria\_current](#method.clear_aria_current)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [auto\_complete](#method.auto_complete)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[AutoComplete](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.AutoComplete.html "enum accesskit::AutoComplete")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [set\_auto\_complete](#method.set_auto_complete)(&mut self, value: [AutoComplete](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.AutoComplete.html "enum accesskit::AutoComplete"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [clear\_auto\_complete](#method.clear_auto_complete)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [has\_popup](#method.has_popup)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[HasPopup](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.HasPopup.html "enum accesskit::HasPopup")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [set\_has\_popup](#method.set_has_popup)(&mut self, value: [HasPopup](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.HasPopup.html "enum accesskit::HasPopup"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [clear\_has\_popup](#method.clear_has_popup)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [list\_style](#method.list_style)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ListStyle](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.ListStyle.html "enum accesskit::ListStyle")\>

The list style type. Only available on list items.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [set\_list\_style](#method.set_list_style)(&mut self, value: [ListStyle](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.ListStyle.html "enum accesskit::ListStyle"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [clear\_list\_style](#method.clear_list_style)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [text\_align](#method.text_align)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TextAlign](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.TextAlign.html "enum accesskit::TextAlign")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [set\_text\_align](#method.set_text_align)(&mut self, value: [TextAlign](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.TextAlign.html "enum accesskit::TextAlign"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [clear\_text\_align](#method.clear_text_align)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [vertical\_offset](#method.vertical_offset)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[VerticalOffset](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.VerticalOffset.html "enum accesskit::VerticalOffset")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [set\_vertical\_offset](#method.set_vertical_offset)(&mut self, value: [VerticalOffset](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/enum.VerticalOffset.html "enum accesskit::VerticalOffset"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2136-2150)

#### pub fn [clear\_vertical\_offset](#method.clear_vertical_offset)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2152-2189)

#### pub fn [transform](#method.transform)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[Affine](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/geometry/struct.Affine.html "struct accesskit::geometry::Affine")\>

An affine transform to apply to any coordinates within this node and its descendants, including the [`bounds`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.bounds "method accesskit::Node::bounds") property of this node. The combined transforms of this node and its ancestors define the coordinate space of this node. /// This should be `None` if it would be set to the identity transform, which should be the case for most nodes.

AccessKit expects the final transformed coordinates to be relative to the origin of the tree’s container (e.g. window), in physical pixels, with the y coordinate being top-down.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2152-2189)

#### pub fn [set\_transform](#method.set_transform)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Affine](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/geometry/struct.Affine.html "struct accesskit::geometry::Affine")\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2152-2189)

#### pub fn [clear\_transform](#method.clear_transform)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2152-2189)

#### pub fn [bounds](#method.bounds)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Rect](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/geometry/struct.Rect.html "struct accesskit::geometry::Rect")\>

The bounding box of this node, in the node’s coordinate space. This property does not affect the coordinate space of either this node or its descendants; only the [`transform`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.transform "method accesskit::Node::transform") property affects that. This, along with the recommendation that most nodes should have a [`transform`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.transform "method accesskit::Node::transform") of `None`, implies that the `bounds` property of most nodes should be in the coordinate space of the nearest ancestor with a non-`None` [`transform`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html#method.transform "method accesskit::Node::transform"), or if there is no such ancestor, the tree’s container (e.g. window).

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2152-2189)

#### pub fn [set\_bounds](#method.set_bounds)(&mut self, value: [Rect](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/geometry/struct.Rect.html "struct accesskit::geometry::Rect"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2152-2189)

#### pub fn [clear\_bounds](#method.clear_bounds)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2152-2189)

#### pub fn [text\_selection](#method.text_selection)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[TextSelection](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.TextSelection.html "struct accesskit::TextSelection")\>

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2152-2189)

#### pub fn [set\_text\_selection](#method.set_text_selection)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[TextSelection](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.TextSelection.html "struct accesskit::TextSelection")\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2152-2189)

#### pub fn [clear\_text\_selection](#method.clear_text_selection)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2152-2189)

#### pub fn [tree\_id](#method.tree_id)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TreeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.TreeId.html "struct accesskit::TreeId")\>

The tree that this node grafts. When set, this node acts as a graft point, and its child is the root of the specified subtree.

A graft node must be created before its subtree is pushed.

Removing a graft node or clearing this property removes its subtree, unless a new graft node is provided in the same update.

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2152-2189)

#### pub fn [set\_tree\_id](#method.set_tree_id)(&mut self, value: [TreeId](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.TreeId.html "struct accesskit::TreeId"))

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2152-2189)

#### pub fn [clear\_tree\_id](#method.clear_tree_id)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2322-2324)

#### pub fn [custom\_actions](#method.custom_actions)(&self) -> &\[[CustomAction](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.CustomAction.html "struct accesskit::CustomAction")\]

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2322-2324)

#### pub fn [set\_custom\_actions](#method.set_custom_actions)(&mut self, value: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[CustomAction](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.CustomAction.html "struct accesskit::CustomAction")\>>)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2322-2324)

#### pub fn [clear\_custom\_actions](#method.clear_custom_actions)(&mut self)

[Source](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/src/accesskit/lib.rs.html#2322-2324)

#### pub fn [push\_custom\_action](#method.push_custom_action)(&mut self, item: [CustomAction](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.CustomAction.html "struct accesskit::CustomAction"))

## Trait Implementations

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

### impl [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

where [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

#### const [STORAGE\_TYPE](../prelude/trait.Component.html#associatedconstant.STORAGE_TYPE): [StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType") = bevy\_ecs::component::StorageType::Table

A constant indicating the storage type used for this component.

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

#### type [Mutability](../prelude/trait.Component.html#associatedtype.Mutability) = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")

A marker type to assist Bevy with determining if this component is mutable, or immutable. Mutable components will have [`Component<Mutability = Mutable>`](../prelude/trait.Component.html "trait bevy::prelude::Component"), while immutable components will instead have [`Component<Mutability = Immutable>`](../prelude/trait.Component.html "trait bevy::prelude::Component"). [Read more](../prelude/trait.Component.html#associatedtype.Mutability)

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

#### fn [register\_required\_components](../prelude/trait.Component.html#method.register_required_components)( \_requiree: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), required\_components: &mut [RequiredComponentsRegistrator](../ecs/component/struct.RequiredComponentsRegistrator.html "struct bevy::ecs::component::RequiredComponentsRegistrator")<'\_, '\_>, )

Registers required components. [Read more](../prelude/trait.Component.html#method.register_required_components)

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

#### fn [clone\_behavior](../prelude/trait.Component.html#method.clone_behavior)() -> [ComponentCloneBehavior](../ecs/component/enum.ComponentCloneBehavior.html "enum bevy::ecs::component::ComponentCloneBehavior")

Called when registering this component, allowing to override clone function (or disable cloning altogether) for this component. [Read more](../prelude/trait.Component.html#method.clone_behavior)

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

#### fn [relationship\_accessor](../prelude/trait.Component.html#method.relationship_accessor)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentRelationshipAccessor](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor")<[AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")\>>

Returns [`ComponentRelationshipAccessor`](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor") required for working with relationships in dynamic contexts. [Read more](../prelude/trait.Component.html#method.relationship_accessor)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#524)

#### fn [on\_add](../prelude/trait.Component.html#method.on_add)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_add` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#529)

#### fn [on\_insert](../prelude/trait.Component.html#method.on_insert)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_insert` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#534)

#### fn [on\_discard](../prelude/trait.Component.html#method.on_discard)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_discard` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#539)

#### fn [on\_remove](../prelude/trait.Component.html#method.on_remove)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_remove` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#544)

#### fn [on\_despawn](../prelude/trait.Component.html#method.on_despawn)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_despawn` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#649)

#### fn [map\_entities](../prelude/trait.Component.html#method.map_entities)<E>(\_this: &mut Self, \_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

Maps the entities on this component using the given [`EntityMapper`](../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"). This is used to remap entities in contexts like scenes and entity cloning. When deriving [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component"), this is populated by annotating fields containing entities with `#[entities]` [Read more](../prelude/trait.Component.html#method.map_entities)

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

### impl [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [Node](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html "struct accesskit::Node")

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

### impl [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#225)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#226)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#226)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>( \_\_deserializer: \_\_D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode"), <\_\_D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#235)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Node](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html "struct accesskit::Node")\> for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#241)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(node: [Node](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html "struct accesskit::Node")) -> [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

Converts an [`accesskit::Node`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html "struct accesskit::Node") into the Bevy Engine [`AccessibilityNode`](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode") newtype.

Doing so allows it to be inserted onto Bevy entities, representing Bevy entities in the `AccessKit` tree.

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#226)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#226)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>( &self, \_\_serializer: \_\_S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#16)

### impl<C> [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") for C

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#17-19)

#### fn [component\_ids](../prelude/trait.Bundle.html#tymethod.component_ids)( components: &mut [ComponentsRegistrator](../ecs/component/struct.ComponentsRegistrator.html "struct bevy::ecs::component::ComponentsRegistrator")<'\_>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\> + use<C>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#23)

#### fn [get\_component\_ids](../prelude/trait.Bundle.html#tymethod.get_component_ids)( components: &[Components](../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>>

Return a iterator over this [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")’s component ids. This will be [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the component has not been registered.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#30)

### impl<C> [BundleFromComponents](../ecs/bundle/trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents") for C

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#31-35)

#### unsafe fn [from\_components](../ecs/bundle/trait.BundleFromComponents.html#tymethod.from_components)<T, F>(ctx: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), func: [&mut F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> C

where F: for<'a> [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'a>, C: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/mod.rs.html#633)

### impl<T> [DeserializeOwned](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.DeserializeOwned.html "trait serde_core::de::DeserializeOwned") for T

where T: for<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#43)

### impl<C> [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for C

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#44)

#### type [Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

An operation on the entity that happens _after_ inserting this bundle.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#46-49)

#### unsafe fn [get\_components](../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)( ptr: [MovingPtr](../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, C>, func: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType"), [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>), ) -> <C as [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect")

Moves the components out of the bundle. [Read more](../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#54)

#### unsafe fn [apply\_effect](../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)( \_ptr: [MovingPtr](../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<C>>, \_entity: &mut [EntityWorldMut](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Applies the after-effects of spawning this bundle. [Read more](../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#722)

### impl<T> [ErasedBundleTemplate](../scene/trait.ErasedBundleTemplate.html "trait bevy::scene::ErasedBundleTemplate") for T

where T: [Template](../prelude/trait.Template.html "trait bevy::prelude::Template") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <T as [Template](../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"): [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#723)

#### unsafe fn [apply](../scene/trait.ErasedBundleTemplate.html#tymethod.apply)( &self, context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Applies this template to the given `entity`. [Read more](../scene/trait.ErasedBundleTemplate.html#tymethod.apply)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#729)

#### fn [clone\_template](../scene/trait.ErasedBundleTemplate.html#tymethod.clone_template)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedBundleTemplate](../scene/trait.ErasedBundleTemplate.html "trait bevy::scene::ErasedBundleTemplate")\>

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#686)

### impl<T> [ErasedComponentTemplate](../scene/trait.ErasedComponentTemplate.html "trait bevy::scene::ErasedComponentTemplate") for T

where T: [Template](../prelude/trait.Template.html "trait bevy::prelude::Template") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <T as [Template](../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"): [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#687-691)

#### unsafe fn [apply](../scene/trait.ErasedComponentTemplate.html#tymethod.apply)( &self, context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, bundle\_writer: &mut [BundleWriter](../ecs/bundle/struct.BundleWriter.html "struct bevy::ecs::bundle::BundleWriter")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Applies this template to the given `entity`. [Read more](../scene/trait.ErasedComponentTemplate.html#tymethod.apply)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#701)

#### fn [clone\_template](../scene/trait.ErasedComponentTemplate.html#tymethod.clone_template)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedComponentTemplate](../scene/trait.ErasedComponentTemplate.html "trait bevy::scene::ErasedComponentTemplate")\>

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#404)

### impl<T> [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#347)

### impl<R, P> [ReadPrimitive](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html "trait lebe::io::ReadPrimitive")<R> for P

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<P>, P: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#377)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#method.clone_into)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

Creates a type-erased clone of this value.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"&\[u8\]":"<h3>Notable traits for <code>&amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for &amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div>","Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}