#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

extern "C" {
  fn tree_sitter_tsx() -> Language;
}

use tree_sitter::{Language, Parser, TreeCursor};

#[napi]
pub fn transform(mut input_code: String) -> String {
  let mut parser = Parser::new();
  let language = unsafe { tree_sitter_tsx() };
  parser.set_language(language).unwrap();
  let tree = parser.parse(&input_code, None).unwrap();
  let mut cursor = tree.walk();
  let bytes = unsafe { input_code.as_bytes_mut() };
  walk_tree(&mut cursor, bytes);
  input_code
}

fn walk_tree(cursor: &mut TreeCursor, bytes: &mut [u8]) {
  loop {
    let node = cursor.node();
    match node.kind() {
      "type_alias_declaration"
      | "type_parameters"
      | "type_parameter"
      | "type_arguments"
      | "type_annotation"
      | "ambient_declaration"
      | "interface_declaration"
      | "implements_clause"
      | "array_type"
      | "union_type"
      | "intersection_type"
      | "tuple_type"
      | "function_type"
      | "generic_type"
      | "constructor_type"
      | "parenthesized_type"
      | "mapped_type_clause"
      | "inferred_type" => {
        replace_byte_range(bytes, node.start_byte(), node.end_byte());
        // Note: no need to traverse children of a type-related node, which will always be type-related
      }
      "import_statement" => {
        let mut should_prune = false;
        match (node.child(1), node.child(2), node.child(3)) {
          (Some(child1), Some(child2), Some(child3)) => {
            match (child1.kind(), child2.kind(), child3.kind()) {
              ("type", "import_clause", "from") => {
                should_prune = true;
              }
              ("import_clause", _, _) => {
                let mut maybe_named_imports = None;
                match (child1.child(0), child1.child(1)) {
                  (Some(node0), Some(node1)) => match (node0.kind(), node1.kind()) {
                    (_, "named_imports") => {
                      maybe_named_imports = Some(node1);
                    }
                    _ => {}
                  },
                  (Some(node0), None) => match node0.kind() {
                    "named_imports" => {
                      maybe_named_imports = Some(node0);
                    }
                    _ => {}
                  },
                  _ => {}
                }
                match maybe_named_imports {
                  Some(named_imports) => {
                    let mut maybe_specifier = named_imports.child(1);
                    let mut non_type_specifier_exists = false;
                    let mut comma_before = None;
                    let mut prev_eliminated = false;
                    loop {
                      match maybe_specifier {
                        Some(specifier) => {
                          match specifier.kind() {
                            "{" | "}" => {
                              prev_eliminated = false;
                            }
                            "," => {
                              if prev_eliminated && comma_before == None {
                                // eliminate trailing comma if no comma before eliminated node
                                replace_byte_range(
                                  bytes,
                                  specifier.start_byte(),
                                  specifier.end_byte(),
                                );
                              }
                              comma_before = Some(specifier);
                              prev_eliminated = false;
                            }
                            "import_specifier" => {
                              if specifier.start_byte()
                                != specifier.named_child(0).unwrap().start_byte()
                              {
                                // Type specifier
                                let mut start_node = specifier;
                                match comma_before {
                                  Some(comma_node) => {
                                    start_node = comma_node;
                                  }
                                  _ => {}
                                }
                                replace_byte_range(
                                  bytes,
                                  start_node.start_byte(),
                                  specifier.end_byte(),
                                );
                                prev_eliminated = true;
                              } else {
                                non_type_specifier_exists = true;
                                prev_eliminated = false;
                              }
                            }
                            _ => {
                              assert_eq!(specifier.kind(), "FIXME");
                            }
                          }
                          maybe_specifier = specifier.next_sibling();
                        }
                        None => {
                          break;
                        }
                      }
                    }
                    if !non_type_specifier_exists {
                      replace_byte_range(bytes, node.start_byte(), node.end_byte());
                    }
                  }
                  None => {}
                }
              }
              _ => {}
            }
          }
          _ => {}
        }
        if should_prune {
          replace_byte_range(bytes, node.start_byte(), node.end_byte());
        }
        if cursor.goto_first_child() {
          // If not a type related node, keep travelling down the tree as far as we can
          continue;
        }
      }
      "export_statement" => {
        let mut should_prune = false;
        match (node.child(1), node.child(2), node.child(3)) {
          (Some(child1), Some(child2), Some(child3)) => {
            match (child1.kind(), child2.kind(), child3.kind()) {
              ("interface_declaration", _, _) => {
                should_prune = true;
              }
              ("type", "export_clause", "from") => {
                should_prune = true;
              }
              ("export_clause", _, _) => {
                let mut maybe_specifier = child1.child(1);
                let mut non_type_specifier_exists = false;
                let mut comma_before = None;
                let mut prev_eliminated = false;
                loop {
                  match maybe_specifier {
                    Some(specifier) => {
                      match specifier.kind() {
                        "{" | "}" => {
                          prev_eliminated = false;
                        }
                        "," => {
                          if prev_eliminated && comma_before == None {
                            // eliminate trailing comma if no comma before eliminated node
                            replace_byte_range(bytes, specifier.start_byte(), specifier.end_byte());
                          }

                          comma_before = Some(specifier);
                          prev_eliminated = false;
                        }
                        "export_specifier" => {
                          if specifier.start_byte()
                            != specifier.named_child(0).unwrap().start_byte()
                          {
                            // Type specifier
                            let mut start_node = specifier;
                            match comma_before {
                              Some(comma_node) => {
                                start_node = comma_node;
                              }
                              _ => {}
                            }
                            replace_byte_range(
                              bytes,
                              start_node.start_byte(),
                              specifier.end_byte(),
                            );
                            prev_eliminated = true;
                          } else {
                            non_type_specifier_exists = true;
                            prev_eliminated = false;
                          }
                        }
                        _ => {
                          assert_eq!(specifier.kind(), "FIXME");
                        }
                      }
                      maybe_specifier = specifier.next_sibling();
                    }
                    None => {
                      break;
                    }
                  }
                }
                if !non_type_specifier_exists {
                  replace_byte_range(bytes, node.start_byte(), node.end_byte());
                }
              }
              _ => {}
            }
          }
          (Some(child1), _, _) => match child1.kind() {
            "interface_declaration" => {
              should_prune = true;
            }
            "type_alias_declaration" => {
              should_prune = true;
            }
            _ => {}
          },
          _ => {}
        }
        if should_prune {
          replace_byte_range(bytes, node.start_byte(), node.end_byte());
        }
        if cursor.goto_first_child() {
          // If not a type related node, keep travelling down the tree as far as we can
          continue;
        }
      }
      "optional_parameter" => {
        match node.child(0) {
          Some(child) => {
            // ...
            match child.kind() {
              "identifier" | "rest_pattern" => {
                replace_byte_range(bytes, child.end_byte(), node.end_byte());
              }
              _ => {}
            }
          }
          _ => {}
        }

        if cursor.goto_first_child() {
          // If not a type related node, keep travelling down the tree as far as we can
          continue;
        }
      }
      _ => {
        if cursor.goto_first_child() {
          // If not a type related node, keep travelling down the tree as far as we can
          continue;
        }
      }
    }

    // If we can't travel any further down, try going to next sibling and repeating
    if cursor.goto_next_sibling() {
      continue;
    }

    // Otherwise, we must travel back up; we'll loop until we reach the root or can
    // go to the next sibling of a node again.
    loop {
      // Since we're retracing back up the tree, this is the last time we'll encounter this node
      if !cursor.goto_parent() {
        // We have arrived back at the root, so we are done.
        return;
      }

      if cursor.goto_next_sibling() {
        // If we succeed in going to the previous node's sibling,
        // we will go back to travelling down that sibling's tree, and we also
        // won't be encountering the previous node again
        break;
      }
    }
  }
}

// Replaces the byte range between start_byte (inclusive) and end_byte (exclusive) in the given byte buffer with whitespace
// Preserving newlines
fn replace_byte_range(bytes: &mut [u8], start_byte: usize, end_byte: usize) {
  for byte in start_byte..end_byte {
    if bytes[byte] != b'\n' {
      bytes[byte] = b' ';
    }
  }
}
