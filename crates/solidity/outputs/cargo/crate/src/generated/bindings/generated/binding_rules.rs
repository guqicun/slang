// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(clippy::needless_raw_string_hashes)]
#[allow(dead_code)] // TODO(#982): use to create the graph
pub const BINDING_RULES_SOURCE: &str = r#####"
    global ROOT_NODE
global FILE_PATH
global JUMP_TO_SCOPE_NODE

attribute node_definition = node     => type = "pop_symbol", node_symbol = node, is_definition
attribute node_reference = node      => type = "push_symbol", node_symbol = node, is_reference
attribute node_symbol = node         => symbol = (source-text node), source_node = node
attribute pop_symbol = symbol        => type = "pop_symbol", symbol = symbol
attribute push_symbol = symbol       => type = "push_symbol", symbol = symbol
attribute symbol_definition = symbol => type = "pop_symbol", symbol = symbol, is_definition
attribute symbol_reference = symbol  => type = "push_symbol", symbol = symbol, is_reference

attribute scoped_node_definition = node => type = "pop_scoped_symbol", node_symbol = node, is_definition
attribute scoped_node_reference  = node => type = "push_scoped_symbol", node_symbol = node, is_reference
attribute pop_scoped_symbol = symbol    => type = "pop_scoped_symbol", symbol = symbol
attribute push_scoped_symbol = symbol   => type = "push_scoped_symbol", symbol = symbol

;; Keeps a link to the enclosing contract definition to provide a parent for
;; method calls (to correctly resolve virtual methods)
inherit .enclosing_def

inherit .parent_scope
inherit .lexical_scope


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Source unit (aka .sol file)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@source_unit [SourceUnit] {
  ;; All lexical_scope nodes eventually connect to the file's root scope
  node @source_unit.lexical_scope

  ;; This provides all the exported symbols from the file
  node @source_unit.defs

  ;; Connect to ROOT_NODE to export our symbols
  node export
  edge ROOT_NODE -> export
  edge export -> @source_unit.defs

  if (is-system-file FILE_PATH) {
    ; If this is a system file (aka. built-ins), export everything through this
    ; special symbol (which is automatically imported below)
    attr (export) pop_symbol = "@@built-ins@@"

  } else {
    ; This is a user file, so we want to export under the file's path symbol
    attr (export) pop_symbol = FILE_PATH

    ; ... and also import the global built-ins
    node built_ins
    attr (built_ins) push_symbol = "@@built-ins@@"

    edge @source_unit.lexical_scope -> built_ins
    edge built_ins -> ROOT_NODE
  }

  let @source_unit.enclosing_def = #null

  ;; This defines a parent_scope at the source unit level (this attribute is
  ;; inherited) for contracts to resolve bases (both in inheritance lists and
  ;; override specifiers)
  let @source_unit.parent_scope = @source_unit.lexical_scope
}

;; Top-level definitions...
@source_unit [SourceUnit [SourceUnitMembers
    [SourceUnitMember @unit_member (
          [ContractDefinition]
        | [InterfaceDefinition]
        | [LibraryDefinition]
        | [StructDefinition]
        | [EnumDefinition]
        | [FunctionDefinition]
        | [ConstantDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
        | [EventDefinition]
    )]
]] {
  edge @unit_member.lexical_scope -> @source_unit.lexical_scope
  edge @source_unit.lexical_scope -> @unit_member.def
  edge @source_unit.defs -> @unit_member.def
}

;; Special case for built-ins: we want to export all symbols in the contract:
;; functions, types and state variables. All built-in symbols are defined in an
;; internal contract named '%BuiltIns%' (renamed from '$BuiltIns$') so we need
;; to export all its members and type members directly as a source unit
;; definition.
;; __SLANG_SOLIDITY_BUILT_INS_CONTRACT_NAME__ keep in sync with built-ins generation.
@source_unit [SourceUnit [SourceUnitMembers
    [SourceUnitMember @contract [ContractDefinition name: ["%BuiltIns%"]]]
]] {
  if (is-system-file FILE_PATH) {
    edge @source_unit.defs -> @contract.members
    edge @source_unit.defs -> @contract.type_members
    edge @source_unit.defs -> @contract.state_vars
  }
}

@source_unit [SourceUnit [SourceUnitMembers [SourceUnitMember @using [UsingDirective]]]] {
  edge @source_unit.lexical_scope -> @using.def
}

@source_unit [SourceUnit [SourceUnitMembers [SourceUnitMember
    @using [UsingDirective [GlobalKeyword]]
]]] {
  ; global using directives are exported by this source unit
  edge @source_unit.defs -> @using.def
}

;; ... and imports
@source_unit [SourceUnit [SourceUnitMembers
     [SourceUnitMember [ImportDirective
         [ImportClause @import (
               [PathImport]
             | [NamedImport]
             | [ImportDeconstruction]
         )]
     ]]
]] {
  node @import.defs
  edge @source_unit.defs -> @import.defs
  edge @source_unit.lexical_scope -> @import.defs
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Imports
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

[ImportClause [_ @path path: [StringLiteral]]] {
  ;; This node represents the imported file and the @path.import node is used by
  ;; all subsequent import rules
  node @path.import
  scan (source-text @path) {
    "^\\s*[\"'](.+)[\"']\\s*$" {
      let resolved_path = (resolve-path FILE_PATH $1)
      attr (@path.import) push_symbol = resolved_path
    }
  }
  edge @path.import -> ROOT_NODE
}

;;; `import <URI>`
@import [PathImport @path path: [StringLiteral] .] {
  ;; This is the "lexical" connection, which makes all symbols exported from the
  ;; imported source unit available for resolution globally at this' source unit
  ;; scope
  edge @import.defs -> @path.import
}

;;; `import <URI> as <IDENT>`
@import [PathImport
   @path path: [StringLiteral]
   alias: [ImportAlias @alias [Identifier]]
] {
  node def
  attr (def) node_definition = @alias
  attr (def) definiens_node = @import
  edge @import.defs -> def

  node member
  attr (member) pop_symbol = "."
  edge def -> member

  ;; Lexical connection, which makes the import available as a member through
  ;; the alias identifier
  edge member -> @path.import
}

;;; `import * as <IDENT> from <URI>`
@import [NamedImport
    alias: [ImportAlias @alias [Identifier]]
    @path path: [StringLiteral]
] {
  node def
  attr (def) node_definition = @alias
  attr (def) definiens_node = @import
  edge @import.defs -> def

  node member
  attr (member) pop_symbol = "."
  edge def -> member

  ;; Lexical connection, which makes the import available as a member through
  ;; the alias identifier
  edge member -> @path.import
}

;;; `import {<SYMBOL> [as <IDENT>] ...} from <PATH>`
@import [ImportDeconstruction
    symbols: [ImportDeconstructionSymbols @symbol [ImportDeconstructionSymbol]]
    @path path: [StringLiteral]
] {
  ;; We define these intermediate nodes for convenience only, to make the
  ;; queries simpler in the two rules below
  node @symbol.def
  edge @import.defs -> @symbol.def

  node @symbol.import
  edge @symbol.import -> @path.import
}

@symbol [ImportDeconstructionSymbol @name name: [Identifier] .] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @symbol
  attr (def) tag = "alias"  ; deprioritize this definition
  edge @symbol.def -> def

  node import
  attr (import) node_reference = @name
  edge def -> import

  edge import -> @symbol.import
}

@symbol [ImportDeconstructionSymbol
    @name name: [Identifier]
    alias: [ImportAlias @alias [Identifier]]
] {
  node def
  attr (def) node_definition = @alias
  attr (def) definiens_node = @symbol
  attr (def) tag = "alias"  ; deprioritize this definition
  edge @symbol.def -> def

  node import
  attr (import) node_reference = @name
  edge def -> import

  edge import -> @symbol.import
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Common inheritance rules (apply to contracts and interfaces)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@specifier [InheritanceSpecifier [InheritanceTypes
    [InheritanceType @type_name [IdentifierPath]]
]] {
  ;; This should point to the enclosing contract or interface definition
  let heir = @specifier.heir

  ;; Resolve base names through the parent scope of our heir (contract or
  ;; interface), aka the source unit
  edge @type_name.push_end -> heir.parent_scope

  ;; Make base members accesible as our own members
  node member
  attr (member) push_symbol = "."

  node typeof
  attr (typeof) push_symbol = "@typeof"

  edge heir.members -> member
  edge member -> typeof
  edge typeof -> @type_name.push_begin

  ;; Make base defs (eg. enums and structs) accessible as our own
  node type_member
  attr (type_member) push_symbol = "."

  edge heir.type_members -> type_member
  edge type_member -> @type_name.push_begin

  ; Resolve the "super" keyword to the inherited type
  edge heir.super -> @type_name.push_begin
}

;; NOTE: we use anchors here to prevent the query engine from returning all the
;; sublists of possible parents
@specifier [InheritanceSpecifier [InheritanceTypes . @parents [_]+ .]] {
  var parent_refs = []
  for parent in @parents {
    if (eq (node-type parent) "InheritanceType") {
      ;; this is intentionally reversed because of how Solidity linearised the contract bases
      set parent_refs = (concat [parent.ref] parent_refs)
    }
  }
  let @specifier.parent_refs = parent_refs
}

@parent [InheritanceType @type_name [IdentifierPath]] {
  let @parent.ref = @type_name.push_begin
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Contracts
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@contract [ContractDefinition] {
  node @contract.lexical_scope
  node @contract.super_scope
  node @contract.def
  node @contract.members
  node @contract.type_members
  node @contract.modifiers
  node @contract.state_vars

  edge @contract.lexical_scope -> @contract.members
  edge @contract.lexical_scope -> @contract.type_members
  edge @contract.lexical_scope -> @contract.state_vars

  ;; Modifiers are available as a contract type members through a special '@modifier' symbol
  node modifier
  attr (modifier) pop_symbol = "@modifier"
  edge @contract.type_members -> modifier
  edge modifier -> @contract.modifiers

  let @contract.enclosing_def = @contract.def
}

@contract [ContractDefinition @name name: [Identifier]] {
  attr (@contract.def) node_definition = @name
  attr (@contract.def) definiens_node = @contract

  ;; "instance" like access path
  ;; we have two distinct paths: @typeof -> . for accesses to variables of the contract's type
  ;; and () -> . for accesses through a `new` invocation (or casting)
  node member
  attr (member) pop_symbol = "."
  edge member -> @contract.members

  node type_def
  attr (type_def) pop_symbol = "@typeof"
  edge @contract.def -> type_def
  edge type_def -> member

  node call
  attr (call) pop_symbol = "()"
  edge @contract.def -> call
  edge call -> member

  ;; "namespace" like access path
  node type_member
  attr (type_member) pop_symbol = "."
  edge @contract.def -> type_member
  edge type_member -> @contract.type_members

  ;; Define "this" and connect it to the contract definition
  node this
  attr (this) pop_symbol = "this"
  edge this -> member

  ;; ... and make it available in the contract's lexical scope
  edge @contract.lexical_scope -> this

  ; Resolve the "this" keyword to the contract itself
  node name_push
  attr (name_push) push_symbol = (source-text @name)
  edge this -> name_push
  edge name_push -> @contract.lexical_scope

  ;; Define "super" effectively as if it was a state variable of a type connected by our super_scope
  ;; super_scope will later connect to the base contract defs directly
  node @contract.super
  attr (@contract.super) pop_symbol = "super"

  node super_typeof
  attr (super_typeof) push_symbol = "@typeof"

  edge @contract.super -> super_typeof
  edge super_typeof -> @contract.super_scope

  ;; Finally make "super" available in the contract's lexical scope for function bodies to use
  edge @contract.lexical_scope -> @contract.super

  ; NOTE: The keyword "super" itself resolves to each of its parent contracts.
  ; See the related rules in the InheritanceSpecifier section above.

  ;; This defines the sink of edges added from base contracts when setting this
  ;; contract as the compilation context
  attr (@contract.def) export_node = @contract.members

  ;; This node will eventually connect to the contract's members being compiled
  ;; and grants access to definitions in that contract and all its parents
  ;; (recursively)
  node super_import
  attr (super_import) pop_symbol = "."
  edge @contract.super -> super_import

  ;; This defines the source side of edges added to base contracts when setting
  ;; a contract as compilation context; this allows this contract (a base) to
  ;; access virtual methods in any sub-contract defined in the hierarchy
  attr (@contract.def) import_nodes = [@contract.lexical_scope, super_import]

  ; Path to resolve the built-in type for type() expressions
  node type
  attr (type) pop_symbol = "%type"
  node type_contract_type
  attr (type_contract_type) push_symbol = "%typeContractType"
  edge @contract.def -> type
  edge type -> type_contract_type
  edge type_contract_type -> @contract.lexical_scope
}

@contract [ContractDefinition @specifier [InheritanceSpecifier]] {
  let @specifier.heir = @contract
  attr (@contract.def) parents = @specifier.parent_refs
}

@contract [ContractDefinition [InheritanceSpecifier [InheritanceTypes
    [InheritanceType @type_name [IdentifierPath]]
]]] {
  ;; The base contract defs are directly accesible through our special super scope
  edge @contract.super_scope -> @type_name.push_begin
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @member (
          [EnumDefinition]
        | [StructDefinition]
        | [EventDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
        | [FunctionDefinition]
        | [ConstructorDefinition]
        | [StateVariableDefinition]
        | [ModifierDefinition]
        | [FallbackFunctionDefinition]
        | [ReceiveFunctionDefinition]
    )]
]] {
  edge @member.lexical_scope -> @contract.lexical_scope
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @using [UsingDirective]]
]] {
  edge @contract.lexical_scope -> @using.def
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @member (
          [EnumDefinition]
        | [StructDefinition]
        | [EventDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
    )]
]] {
  edge @contract.type_members -> @member.def
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @state_var [StateVariableDefinition]]
]] {
  edge @contract.state_vars -> @state_var.def
}

;; Public state variables are also exposed as external member functions
@contract [ContractDefinition [ContractMembers
    [ContractMember @state_var [StateVariableDefinition
        [StateVariableAttributes [StateVariableAttribute [PublicKeyword]]]
    ]]
]] {
  edge @contract.members -> @state_var.def
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @function [FunctionDefinition]]
]] {
  ;; Contract functions are also accessible for an instance of the contract
  edge @contract.members -> @function.def

  ;; This may prioritize this definition (when there are multiple options)
  ;; according to the C3 linerisation ordering
  attr (@function.def) tag = "c3"
  attr (@function.def) parents = [@contract.def]
}

@contract [ContractDefinition [ContractMembers
    [ContractMember @function [FunctionDefinition
        [FunctionAttributes [FunctionAttribute ([ExternalKeyword] | [PublicKeyword])]]
    ]]
]] {
  ; public or external functions are also accessible through the contract type
  edge @contract.type_members -> @function.def
}

@contract [ContractDefinition members: [ContractMembers
    [ContractMember @modifier [ModifierDefinition]]
]] {
  edge @contract.modifiers -> @modifier.def

  ;; This may prioritize this definition (when there are multiple options)
  ;; according to the C3 linerisation ordering
  attr (@modifier.def) tag = "c3"
  attr (@modifier.def) parents = [@contract.def]
}

@override [OverrideSpecifier [OverridePathsDeclaration [OverridePaths
    @base_ident [IdentifierPath]
]]] {
  ;; Resolve overriden bases when listed in the function or modifiers modifiers
  edge @base_ident.push_end -> @override.parent_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Interfaces
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@interface [InterfaceDefinition] {
  node @interface.lexical_scope
  node @interface.def
  node @interface.members
  node @interface.type_members

  edge @interface.lexical_scope -> @interface.members
  edge @interface.lexical_scope -> @interface.type_members
}

@interface [InterfaceDefinition @name name: [Identifier]] {
  attr (@interface.def) node_definition = @name
  attr (@interface.def) definiens_node = @interface

  ;; "instance" like access path
  ;; we have two distinct paths: @typeof -> . for accesses to variables of the contract's type
  ;; and () -> . for accesses through a `new` invocation (or casting)
  node member
  attr (member) pop_symbol = "."
  edge member -> @interface.members

  node typeof
  attr (typeof) pop_symbol = "@typeof"
  edge @interface.def -> typeof
  edge typeof -> member

  node call
  attr (call) pop_symbol = "()"
  edge @interface.def -> call
  edge call -> member

  ;; "namespace" like access path
  node type_member
  attr (type_member) pop_symbol = "."
  edge @interface.def -> type_member
  edge type_member -> @interface.type_members

  ; Path to resolve the built-in type for type() expressions
  node type
  attr (type) pop_symbol = "%type"
  node type_interface_type
  attr (type_interface_type) push_symbol = "%typeInterfaceType"
  edge @interface.def -> type
  edge type -> type_interface_type
  edge type_interface_type -> @interface.lexical_scope
}

@interface [InterfaceDefinition @specifier [InheritanceSpecifier]] {
  let @specifier.heir = @interface
  attr (@interface.def) parents = @specifier.parent_refs

  ; Define a dummy "super" node required by the rules for InheritanceSpecifier
  node @interface.super
}

@interface [InterfaceDefinition [InterfaceMembers
    [ContractMember @member (
          [EnumDefinition]
        | [FunctionDefinition]
        | [StructDefinition]
        | [EventDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
    )]
]] {
  edge @member.lexical_scope -> @interface.lexical_scope
  edge @interface.type_members -> @member.def
}

;; Allow references (eg. variables of the interface type) to the interface to
;; access functions
@interface [InterfaceDefinition members: [InterfaceMembers
    item: [ContractMember @function variant: [FunctionDefinition]]
]] {
  edge @function.lexical_scope -> @interface.lexical_scope
  edge @interface.members -> @function.def
}

[InterfaceDefinition [InterfaceMembers [ContractMember @using [UsingDirective]]]] {
  ; using directives are not allowed in interfaces, but the grammar allows them
  ; so we need to create an artificial node here to connect to created edges from
  ; the internal nodes
  let @using.lexical_scope = (node)
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Libraries
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@library [LibraryDefinition] {
  node @library.lexical_scope
  node @library.def
  node @library.members

  edge @library.lexical_scope -> @library.members
}

@library [LibraryDefinition @name name: [Identifier]] {
  attr (@library.def) node_definition = @name
  attr (@library.def) definiens_node = @library

  node member
  attr (member) pop_symbol = "."
  edge @library.def -> member

  edge member -> @library.members

  ; Path to resolve the built-in type for type() expressions (same as contracts)
  node type
  attr (type) pop_symbol = "%type"
  node type_library_type
  attr (type_library_type) push_symbol = "%typeContractType"
  edge @library.def -> type
  edge type -> type_library_type
  edge type_library_type -> @library.lexical_scope
}

@library [LibraryDefinition [LibraryMembers
    [ContractMember @member (
          [FunctionDefinition]
        | [EnumDefinition]
        | [StructDefinition]
        | [EventDefinition]
        | [ErrorDefinition]
        | [UserDefinedValueTypeDefinition]
    )]
]] {
  edge @member.lexical_scope -> @library.lexical_scope
  edge @library.members -> @member.def
}

@library [LibraryDefinition [LibraryMembers
    [ContractMember @using [UsingDirective]]
]] {
  edge @library.lexical_scope -> @using.def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Using directives
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; The UsingDirective node requires the enclosing context to setup a
;; .lexical_scope scoped variable for it to resolve both targets and subjects.

@using [UsingDirective] {
  ; This node acts as a definition in the sense that provides an entry point
  ; that pops the target type and pushes the library/functions to attach to the
  ; target type
  node @using.def

  ; This internal node connects the other end of the popping path starting at
  ; .def and resolves for the library/functions in the directive
  node @using.clause
}

@using [UsingDirective [UsingClause @id_path [IdentifierPath]]] {
  ; resolve the library to be used in the directive
  edge @id_path.push_end -> @using.lexical_scope

  ; because we're using the whole library, we don't need to "consume" the
  ; attached function (as when using the deconstruction syntax), but we still
  ; need to verify that we're only using this path when resolving a function
  ; access to the target type, not the target type itself
  node dot_guard_pop
  attr (dot_guard_pop) pop_symbol = "."
  node dot_guard_push
  attr (dot_guard_push) push_symbol = "."

  edge @using.clause -> dot_guard_pop
  edge dot_guard_pop -> dot_guard_push
  edge dot_guard_push -> @id_path.push_begin
}

@using [UsingDirective [UsingClause [UsingDeconstruction
    [UsingDeconstructionSymbols [UsingDeconstructionSymbol
        @id_path [IdentifierPath]
    ]]
]]] {
  ; resolve the function to be used in the directive
  edge @id_path.push_end -> @using.lexical_scope

  node dot
  attr (dot) pop_symbol = "."
  node last_identifier
  attr (last_identifier) pop_symbol = (source-text @id_path.rightmost_identifier)

  edge @using.clause -> dot
  edge dot -> last_identifier
  edge last_identifier -> @id_path.push_begin
}

@using [UsingDirective [UsingTarget @type_name [TypeName]]] {
  ; pop the type symbols to connect to the attached function (via @using.clause)
  node typeof
  attr (typeof) pop_symbol = "@typeof"

  edge @using.def -> @type_name.pop_begin
  edge @type_name.pop_end -> typeof
  edge typeof -> @using.clause

  ; resolve the target type of the directive
  edge @type_name.type_ref -> @using.lexical_scope
}

[ContractMember @using [UsingDirective [UsingTarget [Asterisk]]]] {
  ; using X for * is only allowed inside contracts
  node star
  attr (star) pop_symbol = "@*"
  edge @using.def -> star
  edge star -> @using.clause
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Type names
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; TypeName nodes should define two scoped variables:
;;
;; - @type_name.type_ref represents the node in the graph where we're ready to
;;   resolve the type, and thus should generally be connected to a (lexical)
;;   scope node (source node, outside edges connect *from* here).
;;
;; - @type_name.output represents the other end of the type and corresponds to a
;;   state where the type has already been resolved so we can, for example
;;   resolve its members (sink node, outside edges connect *to* here).

@type_name [TypeName @elementary [ElementaryType]] {
  let @type_name.type_ref = @elementary.ref
  let @type_name.output = @elementary.ref
  let @type_name.pop_begin = @elementary.pop
  let @type_name.pop_end = @elementary.pop
}

@type_name [TypeName @id_path [IdentifierPath]] {
  ;; For an identifier path used as a type, the left-most element is the one
  ;; that connects to the parent lexical scope, because the name resolution
  ;; starts at the left of the identifier.
  let @type_name.type_ref = @id_path.push_end

  ;; Conversely, the complete type is found at the right-most name, and that's
  ;; where users of this type should link to (eg. a variable declaration).
  let @type_name.output = @id_path.push_begin

  let @type_name.pop_begin = @id_path.pop_begin
  let @type_name.pop_end = @id_path.pop_end
}

@type_name [TypeName @type_variant ([ArrayTypeName] | [FunctionType])] {
  let @type_name.type_ref = @type_variant.lexical_scope
  let @type_name.output = @type_variant.output
  let @type_name.pop_begin = @type_variant.pop_begin
  let @type_name.pop_end = @type_variant.pop_end
}

@type_name [TypeName @mapping [MappingType]] {
  let @type_name.type_ref = @mapping.lexical_scope
  let @type_name.output = @mapping.output
  let @type_name.pop_begin = @mapping.pop_begin
  let @type_name.pop_end = @mapping.pop_end
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Elementary types
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@elementary [ElementaryType] {
  node @elementary.ref
  attr (@elementary.ref) type = "push_symbol"
  attr (@elementary.ref) source_node = @elementary, symbol = @elementary.symbol

  node @elementary.pop
  attr (@elementary.pop) pop_symbol = @elementary.symbol
}

@elementary [ElementaryType variant: [AddressType @address [AddressKeyword]]] {
  let @elementary.symbol = (format "%{}" (source-text @address))
}

@elementary [ElementaryType @keyword (
      [BoolKeyword]
    | [ByteKeyword]
    | [BytesKeyword]
    | [StringKeyword]
    | [IntKeyword]
    | [UintKeyword]
    | [FixedKeyword]
    | [UfixedKeyword]
)] {
  let @elementary.symbol = (format "%{}" (source-text @keyword))
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Mappings
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@mapping [MappingType] {
  node @mapping.lexical_scope
  node @mapping.output
}

@mapping [MappingType [MappingKey [MappingKeyType @key_ident [IdentifierPath]]]] {
  ; resolve key type
  edge @key_ident.push_end -> @mapping.lexical_scope
}

@mapping [MappingType [MappingValue @value_type [TypeName]]] {
  ; for mapping types we don't need to push the type itself, because we don't need it (yet)
  ; ditto for the pop path, because a mapping type cannot be the target of a using directive

  ; The mapping's type exposes the `%index` (ie. `[]`) operator that returns the value type
  ; This is similar to arrays, only in that case we have a built-in type where
  ; we can define an index function. For mappings we hard-code in the rules directly.

  node typeof_input
  attr (typeof_input) pop_symbol = "@typeof"

  node index_member
  attr (index_member) pop_symbol = "."
  node index
  attr (index) pop_symbol = "%index"
  node index_call
  attr (index_call) pop_symbol = "()"

  node typeof_output
  attr (typeof_output) push_symbol = "@typeof"

  edge @mapping.output -> typeof_input
  edge typeof_input -> index_member
  edge index_member -> index
  edge index -> index_call
  edge index_call -> typeof_output
  edge typeof_output -> @value_type.output

  ; resolve the value type through our scope
  edge @value_type.type_ref -> @mapping.lexical_scope

  ; We use the value_type's definition path as our own because it's needed when
  ; a mapping is the target of a `using` directive. It's not correct, but we
  ; don't have the analog referencing path either.
  let @mapping.pop_begin = @value_type.pop_begin
  let @mapping.pop_end = @value_type.pop_end
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Arrays types
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@array [ArrayTypeName] {
  node @array.lexical_scope
  node @array.output
}

@array [ArrayTypeName [TypeName] index: [Expression]] {
  let @array.type = "%arrayFixed"
}

@array [ArrayTypeName [OpenBracket] . [CloseBracket]] {
  let @array.type = "%array"
}

@array [ArrayTypeName @type_name [TypeName]] {
  ; First define the normal, reference route:

  ; We first push the array type `%array`, which should connect to two distinct paths:
  ; 1. the typed path, which will use a jump scope entry to resolve the element type
  ; 2. the hard-coded path to connect to any `using` directive
  node array
  attr (array) push_symbol = @array.type
  edge @array.output -> array

  ; For the first path, we need to define a scope jump entry for resolving the element type of the array
  node entry
  attr (entry) is_exported
  node element
  attr (element) pop_symbol = "%element"
  edge entry -> element
  edge element -> @type_name.output

  ; And then the path itself
  node params
  attr (params) push_scoped_symbol = "<>", scope = entry
  edge array -> params

  ; Second path, for `using` directives
  edge array -> @type_name.output

  ; Finally, both ends connect to our lexical scope
  edge params -> @array.lexical_scope
  edge @type_name.type_ref -> @array.lexical_scope

  ; Now we define the "definition" route (aka. the pop route), to use in `using` directives only
  ; This is essentially the reverse of the second path above
  node pop_array
  attr (pop_array) pop_symbol = @array.type

  let @array.pop_begin = @type_name.pop_begin
  edge @type_name.pop_end -> pop_array
  let @array.pop_end = pop_array
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Function types
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@ftype [FunctionType @attrs [FunctionTypeAttributes]] {
  ; Compute the built-in type of the function
  ; %functionExternal provides access to .selector and .address
  var type = "%function"
  scan (source-text @attrs) {
    "external" {
      set type = "%functionExternal"
    }
  }

  node @ftype.lexical_scope
  node @ftype.output

  ; This path pushes the function type to the symbol stack
  ; TODO: add parameter and return types to distinguish between different function types
  node function_type
  attr (function_type) push_symbol = type

  edge @ftype.output -> function_type
  edge function_type -> @ftype.lexical_scope

  ; the pop path for the using directive
  node pop_function_type
  attr (pop_function_type) pop_symbol = type

  let @ftype.pop_begin = pop_function_type
  let @ftype.pop_end = pop_function_type
}

@ftype [FunctionType @params [ParametersDeclaration]] {
  edge @params.lexical_scope -> @ftype.lexical_scope
}

@ftype [FunctionType [ReturnsDeclaration @return_params [ParametersDeclaration]]] {
  edge @return_params.lexical_scope -> @ftype.lexical_scope
}

@ftype [FunctionType [ReturnsDeclaration
    [ParametersDeclaration [Parameters . @param [Parameter] .]]
]] {
  ; variables of a function type type can be "called" and resolve to the type of
  ; the return parameter
  node typeof
  attr (typeof) pop_symbol = "@typeof"

  node call
  attr (call) pop_symbol = "()"

  edge @ftype.output -> typeof
  edge typeof -> call
  edge call -> @param.typeof
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Identifier Paths (aka. references to custom types)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; The identifier path builds two graph paths:
;;
;; - From right to left, pushing the identifiers and acting as a "reference".
;;   This path begins at @id_path.push_begin and ends at @id_path.push_end.
;;
;; - From left to right, popping the identifiers (used as a definition sink in
;;   using directives). This path begins at @id_path.pop_begin and ends at
;;   @id_path.pop_end.
;;
;;   NOTE: most of the time, and unless this identifier path is the target of a
;;   using directive this path will not be used and will form a disconnected
;;   graph component. We currently have no way of determining when this path is
;;   necessary, so we always construct it.
;;
;; Additionally the IdentifierPath defines another scoped variable
;; @id_path.rightmost_identifier which corresponds to the identifier in the last
;; position in the path, from left to right. Useful for the using directive to
;; be able to pop the name of the attached function.

@id_path [IdentifierPath @name [Identifier]] {
  node @name.ref
  attr (@name.ref) node_reference = @name
  attr (@name.ref) parents = [@id_path.enclosing_def]

  node @name.pop
  attr (@name.pop) pop_symbol = (source-text @name)
}

@id_path [IdentifierPath @name [Identifier] .] {
  let @id_path.rightmost_identifier = @name

  let @id_path.push_begin = @name.ref
  let @id_path.pop_end = @name.pop
}

[IdentifierPath @left_name [Identifier] . [Period] . @right_name [Identifier]] {
  node ref_member
  attr (ref_member) push_symbol = "."

  edge @right_name.ref -> ref_member
  edge ref_member -> @left_name.ref

  node pop_member
  attr (pop_member) pop_symbol = "."

  edge @left_name.pop -> pop_member
  edge pop_member -> @right_name.pop
}

@id_path [IdentifierPath . @name [Identifier]] {
  let @id_path.push_end = @name.ref
  let @id_path.pop_begin = @name.pop
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Function, parameter declarations and modifiers
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@param [Parameter @type_name [TypeName]] {
  node @param.lexical_scope
  node @param.def

  edge @type_name.type_ref -> @param.lexical_scope

  node @param.typeof
  attr (@param.typeof) push_symbol = "@typeof"
  edge @param.typeof -> @type_name.output
}

@param [Parameter @name [Identifier]] {
  attr (@param.def) node_definition = @name
  attr (@param.def) definiens_node = @param

  edge @param.def -> @param.typeof
}

@params [ParametersDeclaration] {
  node @params.lexical_scope
  node @params.defs

  ;; This scope can be used to resolve named argument calls
  node @params.names
  attr (@params.names) pop_symbol = "@param_names"
  edge @params.names -> @params.defs
}

@params [ParametersDeclaration [Parameters @param item: [Parameter]]] {
  edge @param.lexical_scope -> @params.lexical_scope
  edge @params.defs -> @param.def
}

@function [FunctionDefinition @attrs [FunctionAttributes]] {
  var function_type = "%function"
  scan (source-text @attrs) {
    "\\b(public|external)\\b" {
      set function_type = "%functionExternal"
    }
  }

  node @function.lexical_scope
  node @function.def

  ; this path from the function definition to the scope allows attaching
  ; functions to this function's type
  node typeof
  attr (typeof) push_symbol = "@typeof"
  node type_function
  attr (type_function) push_symbol = function_type
  edge @function.def -> typeof
  edge typeof -> type_function
  edge type_function -> @function.lexical_scope
}

@function [FunctionDefinition name: [FunctionName @name [Identifier]]] {
  attr (@function.def) node_definition = @name
  attr (@function.def) definiens_node = @function
}

@function [FunctionDefinition @params parameters: [ParametersDeclaration]] {
  edge @params.lexical_scope -> @function.lexical_scope

  ;; Input parameters are available in the function scope
  edge @function.lexical_scope -> @params.defs
  ;; ... and shadow other declarations
  attr (@function.lexical_scope -> @params.defs) precedence = 1

  ;; Connect to paramaters for named argument resolution
  edge @function.def -> @params.names
}

@function [FunctionDefinition returns: [ReturnsDeclaration
    @return_params [ParametersDeclaration]
]] {
  edge @return_params.lexical_scope -> @function.lexical_scope

  ;; Return parameters are available in the function scope
  edge @function.lexical_scope -> @return_params.defs
  ;; ... and shadow other declarations
  attr (@function.lexical_scope -> @return_params.defs) precedence = 1
}

;; Only functions that return a single value have an actual return type
;; since tuples are not actual types in Solidity
@function [FunctionDefinition returns: [ReturnsDeclaration
    [ParametersDeclaration [Parameters . @param [Parameter] .]]
]] {
  node call
  attr (call) pop_symbol = "()"

  edge @function.def -> call
  edge call -> @param.typeof
}

;; Connect the function body's block lexical scope to the function
@function [FunctionDefinition [FunctionBody @block [Block]]] {
  edge @block.lexical_scope -> @function.lexical_scope
}

@function [FunctionDefinition [FunctionAttributes item: [FunctionAttribute
    @modifier [ModifierInvocation]
]]] {
  edge @modifier.lexical_scope -> @function.lexical_scope
}

@modifier [ModifierInvocation @name [IdentifierPath]] {
  node @modifier.lexical_scope

  node modifier
  attr (modifier) push_symbol = "@modifier"

  edge @name.push_end -> modifier
  edge modifier -> @modifier.lexical_scope
}

@modifier [ModifierInvocation @args [ArgumentsDeclaration]] {
  edge @args.lexical_scope -> @modifier.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Constructors
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@constructor [ConstructorDefinition] {
  node @constructor.lexical_scope
  node @constructor.def
}

@constructor [ConstructorDefinition @params parameters: [ParametersDeclaration]] {
  edge @params.lexical_scope -> @constructor.lexical_scope

  ;; Input parameters are available in the constructor scope
  edge @constructor.lexical_scope -> @params.defs
  ;; ... and shadow other declarations
  attr (@constructor.lexical_scope -> @params.defs) precedence = 1

  ;; Connect to paramaters for named argument resolution
  edge @constructor.def -> @params.names
}

;; Connect the constructor body's block lexical scope to the constructor
@constructor [ConstructorDefinition @block [Block]] {
  edge @block.lexical_scope -> @constructor.lexical_scope
}

@constructor [ConstructorDefinition [ConstructorAttributes item: [ConstructorAttribute
    @modifier [ModifierInvocation]
]]] {
  edge @modifier.lexical_scope -> @constructor.lexical_scope
}

@contract [ContractDefinition [ContractMembers [ContractMember
    @constructor [ConstructorDefinition]
]]] {
  ;; This link allows calling a constructor with the named parameters syntax
  edge @contract.def -> @constructor.def
}

;; Solidity < 0.5.0 constructors were declared as functions of the contract's name
@contract [ContractDefinition
    @contract_name [Identifier]
    [ContractMembers [ContractMember [FunctionDefinition
        [FunctionName @function_name [Identifier]]
        @params [ParametersDeclaration]
    ]]]
] {
  if (version-matches "< 0.5.0") {
    if (eq (source-text @contract_name) (source-text @function_name)) {
      ; Connect to paramaters for named argument resolution
      edge @contract.def -> @params.names
    }
  }
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Fallback and receive functions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@fallback [FallbackFunctionDefinition] {
  node @fallback.lexical_scope
}

@fallback [FallbackFunctionDefinition @params parameters: [ParametersDeclaration]] {
  edge @params.lexical_scope -> @fallback.lexical_scope

  ;; Input parameters are available in the fallback function scope
  edge @fallback.lexical_scope -> @params.defs
  attr (@fallback.lexical_scope -> @params.defs) precedence = 1
}

@fallback [FallbackFunctionDefinition returns: [ReturnsDeclaration
    @return_params [ParametersDeclaration]
]] {
  edge @return_params.lexical_scope -> @fallback.lexical_scope

  ;; Return parameters are available in the fallback function scope
  edge @fallback.lexical_scope -> @return_params.defs
  attr (@fallback.lexical_scope -> @return_params.defs) precedence = 1
}

@fallback [FallbackFunctionDefinition [FunctionBody @block [Block]]] {
  edge @block.lexical_scope -> @fallback.lexical_scope
}

@fallback [FallbackFunctionDefinition [FallbackFunctionAttributes
    item: [FallbackFunctionAttribute @modifier [ModifierInvocation]]
]] {
  edge @modifier.lexical_scope -> @fallback.lexical_scope
}

@receive [ReceiveFunctionDefinition] {
  node @receive.lexical_scope
}

@receive [ReceiveFunctionDefinition [FunctionBody @block [Block]]] {
  edge @block.lexical_scope -> @receive.lexical_scope
}

@receive [ReceiveFunctionDefinition [ReceiveFunctionAttributes
    item: [ReceiveFunctionAttribute @modifier [ModifierInvocation]]
]] {
  edge @modifier.lexical_scope -> @receive.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Function modifiers
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@modifier [ModifierDefinition] {
  node @modifier.def
  node @modifier.lexical_scope
}

@modifier [ModifierDefinition
    @name name: [Identifier]
    body: [FunctionBody @body [Block]]
] {
  attr (@modifier.def) node_definition = @name
  attr (@modifier.def) definiens_node = @modifier

  edge @body.lexical_scope -> @modifier.lexical_scope

  ; Special case: bind the place holder statement `_` to the built-in
  ; `%placeholder`. This only happens in the body of a modifier.
  node placeholder_pop
  attr (placeholder_pop) pop_symbol = "_"
  node placeholder_ref
  attr (placeholder_ref) push_symbol = "%placeholder"

  edge @body.lexical_scope -> placeholder_pop
  edge placeholder_pop -> placeholder_ref
  edge placeholder_ref -> @modifier.lexical_scope
}

@modifier [ModifierDefinition @params [ParametersDeclaration]] {
  edge @params.lexical_scope -> @modifier.lexical_scope

  ;; Input parameters are available in the modifier scope
  edge @modifier.lexical_scope -> @params.defs
  attr (@modifier.lexical_scope -> @params.defs) precedence = 1
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Blocks and generic statements
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@block [Block] {
  node @block.lexical_scope
  node @block.defs
}

;; The first statement in a block
@block [Block [Statements . @stmt [Statement]]] {
  if (version-matches ">= 0.5.0") {
    edge @stmt.lexical_scope -> @block.lexical_scope
  }
}

@block [Block [Statements @stmt [Statement]]] {
  ;; Hoist statement definitions for Solidity < 0.5.0
  if (version-matches "< 0.5.0") {
    ;; definitions are carried over to the block
    edge @block.defs -> @stmt.defs

    ;; resolution happens in the context of the block
    edge @stmt.lexical_scope -> @block.lexical_scope

    ;; and the statement definitions are available block's scope
    edge @block.lexical_scope -> @stmt.defs
    ;; ... shadowing declarations in enclosing scopes
    attr (@block.lexical_scope -> @stmt.defs) precedence = 1
  }
}

;; Two consecutive statements
[Statements @left_stmt [Statement] . @right_stmt [Statement]] {
  if (version-matches ">= 0.5.0") {
    edge @right_stmt.lexical_scope -> @left_stmt.lexical_scope
  }
}

@stmt [Statement] {
  node @stmt.lexical_scope
  node @stmt.defs

  if (version-matches ">= 0.5.0") {
    ;; For Solidity >= 0.5.0, definitions are immediately available in the
    ;; statement scope. For < 0.5.0 this is also true, but resolved through the
    ;; enclosing block's lexical scope.
    edge @stmt.lexical_scope -> @stmt.defs
    ;; Statement definitions shadow other declarations in its scope
    attr (@stmt.lexical_scope -> @stmt.defs) precedence = 1
  }
}

;; Statements of type block
@stmt [Statement @block variant: [Block]] {
  edge @block.lexical_scope -> @stmt.lexical_scope

  ;; Hoist block definitions (< 0.5.0)
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @block.defs
  }
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Expressions & declaration statements
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; In general for statements the structure is [Statement [StmtVariant]] and we
;; will define the scoped nodes .lexical_scope and (possibly) .defs in the
;; Statement CST node, skipping scoped nodes in the variant of the statement.
;;
;; For expression statements, variable and tuple declarations we define them
;; separately from the enclosing statement to be able to use them in `for`
;; initialization and condition clauses directly. Also, because we intend to
;; reuse them, all of them must have both a .lexical_scope and .defs scoped
;; nodes (even though .defs doesn't make sense for ExpressionStatement)

@stmt [Statement @expr_stmt [ExpressionStatement]] {
  edge @expr_stmt.lexical_scope -> @stmt.lexical_scope
}

@expr_stmt [ExpressionStatement] {
  node @expr_stmt.lexical_scope
  node @expr_stmt.defs
}


;;; Variable declaration statements

@stmt [Statement @var_decl [VariableDeclarationStatement]] {
  edge @var_decl.lexical_scope -> @stmt.lexical_scope
  edge @stmt.defs -> @var_decl.defs
}

@var_decl [VariableDeclarationStatement] {
  node @var_decl.lexical_scope
  node @var_decl.defs
}

@var_decl [VariableDeclarationStatement
    [VariableDeclarationType @var_type [TypeName]]
    @name name: [Identifier]
] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @var_decl

  edge @var_decl.defs -> def
  edge @var_type.type_ref -> @var_decl.lexical_scope

  node typeof
  attr (typeof) push_symbol = "@typeof"

  edge def -> typeof
  edge typeof -> @var_type.output
}


;;; Tuple deconstruction statements

@stmt [Statement @tuple_decon [TupleDeconstructionStatement]] {
  edge @tuple_decon.lexical_scope -> @stmt.lexical_scope
  edge @stmt.defs -> @tuple_decon.defs
}

@tuple_decon [TupleDeconstructionStatement] {
  node @tuple_decon.lexical_scope
  node @tuple_decon.defs
}

@tuple_decon [TupleDeconstructionStatement [TupleDeconstructionElements
    [TupleDeconstructionElement
        @tuple_member [TupleMember variant: [UntypedTupleMember
            @name name: [Identifier]]
        ]
    ]
]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @tuple_member

  edge @tuple_decon.defs -> def
}

@tuple_decon [TupleDeconstructionStatement [TupleDeconstructionElements
    [TupleDeconstructionElement
        @tuple_member [TupleMember variant: [TypedTupleMember
            @member_type type_name: [TypeName]
            @name name: [Identifier]]
        ]
    ]
]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @tuple_member

  edge @tuple_decon.defs -> def
  edge @member_type.type_ref -> @tuple_decon.lexical_scope

  node typeof
  attr (typeof) push_symbol = "@typeof"

  edge def -> typeof
  edge typeof -> @member_type.output
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Control statements
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; If conditionals

@stmt [Statement [IfStatement @body body: [Statement]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @body.defs
  }
}

@stmt [Statement [IfStatement else_branch: [ElseBranch @else_body body: [Statement]]]] {
  edge @else_body.lexical_scope -> @stmt.lexical_scope
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @else_body.defs
  }
}

;; For loops

@stmt [Statement [ForStatement
    initialization: [ForStatementInitialization
        @init_stmt ([ExpressionStatement]
                  | [VariableDeclarationStatement]
                  | [TupleDeconstructionStatement])
    ]
]] {
  edge @init_stmt.lexical_scope -> @stmt.lexical_scope
  edge @stmt.init_defs -> @init_stmt.defs
}

@stmt [Statement [ForStatement
    condition: [ForStatementCondition @cond_stmt [ExpressionStatement]]
]] {
  edge @cond_stmt.lexical_scope -> @stmt.lexical_scope
  edge @cond_stmt.lexical_scope -> @stmt.init_defs
}

@stmt [Statement [ForStatement @iter_expr iterator: [Expression]]] {
  ; for the iterator expression we need an independent scope node that can
  ; connect to both the for-statement *and* the definitions in the init
  ; expression
  node @iter_expr.lexical_scope
  edge @iter_expr.lexical_scope -> @stmt.lexical_scope
  edge @iter_expr.lexical_scope -> @stmt.init_defs
}

@stmt [Statement [ForStatement @body body: [Statement]]] {
  node @stmt.init_defs

  edge @body.lexical_scope -> @stmt.lexical_scope
  edge @body.lexical_scope -> @stmt.init_defs
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @body.defs
    edge @stmt.defs -> @stmt.init_defs
  }
}

;; While loops

@stmt [Statement [WhileStatement @body body: [Statement]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @body.defs
  }
}

;; Do-while loops

@stmt [Statement [DoWhileStatement @body body: [Statement]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
  if (version-matches "< 0.5.0") {
    edge @stmt.defs -> @body.defs
  }
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Error handling
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;;; Try-catch statements

@stmt [Statement [TryStatement @body body: [Block]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
}

@stmt [Statement [TryStatement
    [ReturnsDeclaration @return_params [ParametersDeclaration]]
    @body body: [Block]
]] {
  edge @return_params.lexical_scope -> @stmt.lexical_scope
  edge @body.lexical_scope -> @return_params.defs
  ;; Similar to functions, return params shadow other declarations
  attr (@body.lexical_scope -> @return_params.defs) precedence = 1
}

@stmt [Statement [TryStatement [CatchClauses [CatchClause
    @body body: [Block]
]]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
}

@stmt [Statement [TryStatement [CatchClauses [CatchClause
    [CatchClauseError @catch_params parameters: [ParametersDeclaration]]
    @body body: [Block]
]]]] {
  edge @catch_params.lexical_scope -> @stmt.lexical_scope
  edge @body.lexical_scope -> @catch_params.defs
  ;; Similar to functions, catch params shadow other declarations
  attr (@body.lexical_scope -> @catch_params.defs) precedence = 1
}


;;; Revert statements

@stmt [Statement [RevertStatement @error_ident [IdentifierPath]]] {
  edge @error_ident.push_end -> @stmt.lexical_scope
}

@stmt [Statement [RevertStatement @args [ArgumentsDeclaration]]] {
  edge @args.lexical_scope -> @stmt.lexical_scope
}

[Statement [RevertStatement
    @error_ident [IdentifierPath]
    @args [ArgumentsDeclaration]
]] {
  edge @args.refs -> @error_ident.push_begin
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Other statements
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;;; Emit
@stmt [Statement [EmitStatement
    @event_ident [IdentifierPath]
    @args [ArgumentsDeclaration]
]] {
  edge @event_ident.push_end -> @stmt.lexical_scope
  edge @args.lexical_scope -> @stmt.lexical_scope
  edge @args.refs -> @event_ident.push_begin
}

;;; Unchecked
@stmt [Statement [UncheckedBlock @block block: [Block]]] {
  edge @block.lexical_scope -> @stmt.lexical_scope
}

;;; Assembly
@stmt [Statement [AssemblyStatement @body body: [YulBlock]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; State Variables
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@state_var [StateVariableDefinition] {
  node @state_var.lexical_scope
  node @state_var.def
}

@state_var [StateVariableDefinition
    @type_name type_name: [TypeName]
    @name name: [Identifier]
] {
  attr (@state_var.def) node_definition = @name
  attr (@state_var.def) definiens_node = @state_var

  edge @type_name.type_ref -> @state_var.lexical_scope

  node typeof
  attr (typeof) push_symbol = "@typeof"

  edge @state_var.def -> typeof
  edge typeof -> @type_name.output
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Enum definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@enum [EnumDefinition] {
  node @enum.lexical_scope
  node @enum.def
  node @enum.members
}

@enum [EnumDefinition @name name: [Identifier]] {
  attr (@enum.def) node_definition = @name
  attr (@enum.def) definiens_node = @enum

  node member
  attr (member) pop_symbol = "."

  edge @enum.def -> member
  edge member -> @enum.members

  ; Path to resolve the built-in type for enums (which is the same as for integer types)
  node type
  attr (type) pop_symbol = "%type"
  node type_enum_type
  attr (type_enum_type) push_symbol = "%typeIntType"
  edge @enum.def -> type
  edge type -> type_enum_type
  edge type_enum_type -> @enum.lexical_scope
}

@enum [EnumDefinition
    members: [EnumMembers @item [Identifier]]
] {
  node def
  attr (def) node_definition = @item
  attr (def) definiens_node = @item

  edge @enum.members -> def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Structure definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@struct [StructDefinition] {
  node @struct.lexical_scope
  node @struct.def
  node @struct.members
}

@struct [StructDefinition @name name: [Identifier]] {
  ; Since we use structs to define built-in types and some of them (ie. array)
  ; have have a parametric type, we define two distinct paths to define a
  ; struct:
  ; 1. the normal, non parametric path, should drop scopes in the scope stack first of all
  ; 2. the parametric path, that pops a scope to resolve the parametric type
  ; Both of these connect to the node that pops the struct identifier symbol

  ; First the normal path
  node struct_drop
  attr (struct_drop) type = "drop_scopes"
  edge @struct.def -> struct_drop

  ; Second path, pops the scope
  node typed_params
  attr (typed_params) pop_scoped_symbol = "<>"
  edge @struct.def -> typed_params

  ; Connect both to the struct identifier
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @struct
  edge struct_drop -> def
  edge typed_params -> def

  ; On the other end, to properly close the second path we need to jump to the popped scope
  ; (this is why on the other path we drop scopes)
  edge @struct.lexical_scope -> JUMP_TO_SCOPE_NODE

  ; Now connect normally to the struct members
  node type_def
  attr (type_def) pop_symbol = "@typeof"
  node member
  attr (member) pop_symbol = "."
  edge def -> type_def
  edge type_def -> member
  edge member -> @struct.members

  ; Bind member names when using construction with named arguments
  node param_names
  attr (param_names) pop_symbol = "@param_names"
  edge def -> param_names
  edge param_names -> @struct.members
}

@struct [StructDefinition [StructMembers
    @member item: [StructMember @type_name [TypeName] @name name: [Identifier]]
]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @member

  edge @struct.members -> def

  edge @type_name.type_ref -> @struct.lexical_scope

  node typeof
  attr (typeof) push_symbol = "@typeof"

  edge def -> typeof
  edge typeof -> @type_name.output
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Event definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@event [EventDefinition @name name: [Identifier]] {
  node @event.lexical_scope
  node @event.def

  attr (@event.def) node_definition = @name
  attr (@event.def) definiens_node = @event

  node @event.params
  attr (@event.params) pop_symbol = "@param_names"
  edge @event.def -> @event.params
}

@event [EventDefinition [EventParametersDeclaration [EventParameters
    [EventParameter @type_name type_name: [TypeName]]
]]] {
  edge @type_name.type_ref -> @event.lexical_scope
}

@event [EventDefinition [EventParametersDeclaration [EventParameters
    @param [EventParameter
        @name name: [Identifier]
    ]
]]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @param

  edge @event.params -> def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Error definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@error [ErrorDefinition @name name: [Identifier]] {
  node @error.lexical_scope
  node @error.def

  attr (@error.def) node_definition = @name
  attr (@error.def) definiens_node = @error

  node @error.params
  attr (@error.params) pop_symbol = "@param_names"
  edge @error.def -> @error.params
}

@error [ErrorDefinition [ErrorParametersDeclaration [ErrorParameters
    [ErrorParameter @type_name type_name: [TypeName]]
]]] {
    edge @type_name.type_ref -> @error.lexical_scope
}

@error [ErrorDefinition [ErrorParametersDeclaration [ErrorParameters
    @param [ErrorParameter
        @name name: [Identifier]
    ]
]]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @param

  edge @error.params -> def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Other named definitions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

@constant [ConstantDefinition] {
  node @constant.lexical_scope
  node @constant.def
}

@constant [ConstantDefinition
    @type_name type_name: [TypeName]
    @name name: [Identifier]
] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @constant

  edge @constant.def -> def

  edge @type_name.type_ref -> @constant.lexical_scope
}

@user_type [UserDefinedValueTypeDefinition] {
  node @user_type.lexical_scope
  node @user_type.def
}

@user_type [UserDefinedValueTypeDefinition @name [Identifier]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @user_type

  edge @user_type.def -> def
}


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Expressions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; Expressions have two important scoped variables:
;; - @expr.lexical_scope should be set by the enclosing node to provide a scope
;;   for name resolution
;; - @expr.output is a node provided by the expression and represents the output
;;   of the expression for chaining eg. with a member access

@expr [Expression] {
  ;; this is an output scope for use in member access (and other uses)
  node @expr.output
}

;; Identifier expressions
@expr [Expression @name (
  variant: [Identifier] | variant: [SuperKeyword] | variant: [ThisKeyword] 
)] {
  node ref
  attr (ref) node_reference = @name
  attr (ref) parents = [@expr.enclosing_def]

  edge ref -> @expr.lexical_scope
  edge @expr.output -> ref
}

;; Member access expressions
@expr [Expression [MemberAccessExpression
    @operand operand: [Expression]
    @name member: [Identifier]
]] {
  node @name.ref
  attr (@name.ref) node_reference = @name
  attr (@name.ref) parents = [@expr.enclosing_def]

  node member
  attr (member) push_symbol = "."

  edge @name.ref -> member
  edge member -> @operand.output

  edge @expr.output -> @name.ref

  ; Shortcut path for expressions inside contracts with using X for * directives
  node star
  attr (star) push_symbol = "@*"
  edge member -> star
  edge star -> @expr.lexical_scope
}

;; Special case: member accesses to `super` are tagged with "super" to rank
;; virtual methods correctly
[MemberAccessExpression
    operand: [Expression [SuperKeyword]]
    @name member: [Identifier]
] {
  attr (@name.ref) tag = "super"
}

;; Index access expressions
@expr [Expression [IndexAccessExpression
    @operand operand: [Expression]
]] {
  node index_call
  attr (index_call) push_symbol = "()"
  node index
  attr (index) push_symbol = "%index"
  node index_member
  attr (index_member) push_symbol = "."

  edge @expr.output -> index_call
  edge index_call -> index
  edge index -> index_member
  edge index_member -> @operand.output
}

;; Type expressions
@type_expr [Expression [TypeExpression @type [TypeName]]] {
  edge @type.type_ref -> @type_expr.lexical_scope
}

@type_expr [Expression [TypeExpression [TypeName [ElementaryType ([IntKeyword] | [UintKeyword])]]]] {
  ; For integer types the type's type is fixed
  node typeof
  attr (typeof) push_symbol = "@typeof"
  node type
  attr (type) push_symbol = "%typeIntType"

  edge @type_expr.output -> typeof
  edge typeof -> type
  edge type -> @type_expr.lexical_scope
}

@type_expr [Expression [TypeExpression [TypeName @id_path [IdentifierPath]]]] {
  ; For other identifiers, resolve it through a pseudo-member `%type`
  node typeof
  attr (typeof) push_symbol = "@typeof"
  node type
  attr (type) push_symbol = "%type"

  edge @type_expr.output -> typeof
  edge typeof -> type
  edge type -> @id_path.push_begin
}

;; New expressions

@new_expr [Expression [NewExpression @type [TypeName]]] {
  edge @type.type_ref -> @new_expr.lexical_scope
  edge @new_expr.output -> @type.output
}


;;; Function call expressions

@args [ArgumentsDeclaration] {
  node @args.lexical_scope

  node @args.refs
  attr (@args.refs) push_symbol = "@param_names"
}

@named_arg [NamedArgument @name [Identifier] [Colon] [Expression]] {
  node @named_arg.lexical_scope

  node @named_arg.ref
  attr (@named_arg.ref) node_reference = @name
}

@args [ArgumentsDeclaration [NamedArgumentsDeclaration
    [NamedArgumentGroup [NamedArguments @argument [NamedArgument]]]
]] {
  edge @argument.lexical_scope -> @args.lexical_scope
  edge @argument.ref -> @args.refs
}

@funcall [Expression [FunctionCallExpression
    @operand [Expression]
    @args [ArgumentsDeclaration]
]] {
  edge @args.lexical_scope -> @funcall.lexical_scope

  ;; Connect to the output of the function name to be able to resolve named arguments
  edge @args.refs -> @operand.output

  node call
  attr (call) push_symbol = "()"

  edge @funcall.output -> call
  edge call -> @operand.output
}


;;; Call options

@expr [Expression [CallOptionsExpression @operand [Expression] @options [CallOptions]]] {
  edge @expr.output -> @operand.output

  node @options.refs
  attr (@options.refs) push_symbol = "@param_names"

  node call_options
  attr (call_options) push_symbol = "%callOptions"

  edge @options.refs -> call_options
  edge call_options -> @expr.lexical_scope
}

@expr [Expression [CallOptionsExpression
    @options [CallOptions @named_arg [NamedArgument]]
]] {
  edge @named_arg.lexical_scope -> @expr.lexical_scope
  edge @named_arg.ref -> @options.refs
}



;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;; Yul
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;;; Blocks and statements

@block [YulBlock] {
  node @block.lexical_scope
  ; Variables defined in this block (only used to forward the init block
  ; declarations in a for statement)
  node @block.variable_defs
  ; Function definitions accessible from the block (ie. defined in the block, or
  ; accessible in the enclosing parent block)
  node @block.function_defs

  edge @block.lexical_scope -> @block.function_defs
}

@block [YulBlock [YulStatements . @stmt [YulStatement]]] {
  edge @stmt.lexical_scope -> @block.lexical_scope
}

@block [YulBlock [YulStatements @stmt [YulStatement]]] {
  edge @stmt.function_scope -> @block.function_defs
  edge @block.variable_defs -> @stmt.defs
}

[YulStatements @left_stmt [YulStatement] . @right_stmt [YulStatement]] {
  edge @right_stmt.lexical_scope -> @left_stmt.lexical_scope
  ; variable declaration are accessible from the next statement
  edge @right_stmt.lexical_scope -> @left_stmt.defs
}

@stmt [YulStatement] {
  node @stmt.lexical_scope
  node @stmt.defs
  ;; Functions visible in this scope (to propagate to inner function
  ;; definitions, since the lexical scope is not accessible inside a function
  ;; body)
  node @stmt.function_scope
}

;;; Blocks as statements

@stmt [YulStatement @block variant: [YulBlock]] {
  edge @block.lexical_scope -> @stmt.lexical_scope
  edge @block.function_defs -> @stmt.function_scope
}

;;; Expression as statements

@stmt [YulStatement @expr_stmt [YulExpression]] {
  edge @expr_stmt.lexical_scope -> @stmt.lexical_scope
}

;;; Variable declarations

@stmt [YulStatement @var_decl [YulVariableDeclarationStatement]] {
  edge @var_decl.lexical_scope -> @stmt.lexical_scope
  edge @stmt.defs -> @var_decl.defs
}

@var_decl [YulVariableDeclarationStatement] {
  node @var_decl.lexical_scope
  node @var_decl.defs
}

@var_decl [YulVariableDeclarationStatement [YulVariableNames @name [YulIdentifier]]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @var_decl

  edge @var_decl.defs -> def
}

@var_decl [YulVariableDeclarationStatement [YulVariableDeclarationValue
    @value [YulExpression]
]] {
  edge @value.lexical_scope -> @var_decl.lexical_scope
}

;;; Variable assignments

@stmt [YulStatement @var_assign [YulVariableAssignmentStatement]] {
  edge @var_assign.lexical_scope -> @stmt.lexical_scope
}

@var_assign [YulVariableAssignmentStatement] {
  node @var_assign.lexical_scope
}

@var_assign [YulVariableAssignmentStatement [YulPaths @path [YulPath]]] {
  edge @path.lexical_scope -> @var_assign.lexical_scope
}

@var_assign [YulVariableAssignmentStatement @expr expression: [YulExpression]] {
  edge @expr.lexical_scope -> @var_assign.lexical_scope
}

;;; Function definitions

@block [YulBlock [YulStatements [YulStatement @fundef [YulFunctionDefinition]]]] {
  ;; Function definitions are hoisted in the enclosing block
  edge @block.function_defs -> @fundef.def
  ;; The only definitions available in the function's lexical scope (other than
  ;; parameters) are functions (ie. the body of the function doesn't have access
  ;; to any outside variables)
  edge @fundef.lexical_scope -> @block.function_defs
}

@fundef [YulFunctionDefinition
    @name name: [YulIdentifier]
    @body body: [YulBlock]
] {
  node @fundef.lexical_scope
  node @fundef.def

  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @fundef

  edge @fundef.def -> def
  edge @body.lexical_scope -> @fundef.lexical_scope
}

@fundef [YulFunctionDefinition [YulParametersDeclaration [YulParameters
    @param [YulIdentifier]
]]] {
  node def
  attr (def) node_definition = @param
  attr (def) definiens_node = @param

  edge @fundef.lexical_scope -> def
}

@fundef [YulFunctionDefinition [YulReturnsDeclaration [YulVariableNames
    @return_param [YulIdentifier]
]]] {
  node def
  attr (def) node_definition = @return_param
  attr (def) definiens_node = @return_param

  edge @fundef.lexical_scope -> def
}

;;; Stack assignment (Solidity < 0.5.0)

@stmt [YulStatement [YulStackAssignmentStatement @name [YulIdentifier]]] {
  node ref
  attr (ref) node_reference = @name

  edge ref -> @stmt.lexical_scope
}

;;; If statements

@stmt [YulStatement [YulIfStatement
    @condition condition: [YulExpression]
    @body body: [YulBlock]
]] {
  edge @condition.lexical_scope -> @stmt.lexical_scope
  edge @body.lexical_scope -> @stmt.lexical_scope
  edge @body.function_defs -> @stmt.function_scope
}

;;; Switch statements

@stmt [YulStatement [YulSwitchStatement
    @expr expression: [YulExpression]
]] {
  edge @expr.lexical_scope -> @stmt.lexical_scope
}

@stmt [YulStatement [YulSwitchStatement [YulSwitchCases [YulSwitchCase
    [_ @body body: [YulBlock]]
]]]] {
  edge @body.lexical_scope -> @stmt.lexical_scope
  edge @body.function_defs -> @stmt.function_scope
}

;;; For statements

@stmt [YulStatement [YulForStatement
    @init initialization: [YulBlock]
    @cond condition: [YulExpression]
    @iter iterator: [YulBlock]
    @body body: [YulBlock]
]] {
  edge @init.lexical_scope -> @stmt.lexical_scope
  edge @cond.lexical_scope -> @stmt.lexical_scope
  edge @iter.lexical_scope -> @stmt.lexical_scope
  edge @body.lexical_scope -> @stmt.lexical_scope

  edge @cond.lexical_scope -> @init.variable_defs
  edge @iter.lexical_scope -> @init.variable_defs
  edge @body.lexical_scope -> @init.variable_defs
}

;;; Label statements (Solidity < 0.5.0)

@block [YulBlock [YulStatements [YulStatement @label [YulLabel @name label: [YulIdentifier]]]]] {
  node def
  attr (def) node_definition = @name
  attr (def) definiens_node = @label

  ; Labels are hoisted to the beginning of the block
  edge @block.lexical_scope -> def
}

;;; Expressions

@expr [YulExpression] {
  node @expr.lexical_scope
}

@expr [YulExpression @path [YulPath]] {
  edge @path.lexical_scope -> @expr.lexical_scope
}

@path [YulPath] {
  node @path.lexical_scope
}

@path [YulPath @name [YulIdentifier]] {
  node ref
  attr (ref) node_reference = @name

  edge ref -> @path.lexical_scope
}

@expr [YulExpression @funcall [YulFunctionCallExpression]] {
  edge @funcall.lexical_scope -> @expr.lexical_scope
}

@funcall [YulFunctionCallExpression
  @operand operand: [YulExpression]
  @args arguments: [YulArguments]
] {
  node @funcall.lexical_scope

  edge @operand.lexical_scope -> @funcall.lexical_scope
  edge @args.lexical_scope -> @funcall.lexical_scope
}

@args [YulArguments] {
  node @args.lexical_scope
}

@args [YulArguments @arg [YulExpression]] {
  edge @arg.lexical_scope -> @args.lexical_scope
}

"#####;
