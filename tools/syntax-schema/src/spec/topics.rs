use super::productions::{write_production, SpecProductionContext};
use super::NavigationEntry;
use crate::schema::Grammar;
use std::io::Write;
use std::{fs::File, path::PathBuf};

pub fn generate_spec_sections(
    grammar: &Grammar,
    generated_folder: &PathBuf,
    entries: &mut Vec<NavigationEntry>,
) {
    let context = generate_context(grammar);

    grammar
        .manifest
        .sections
        .iter()
        .enumerate()
        .for_each(|(section_index, section)| {
            entries.push(NavigationEntry {
                indentation_level: 1,
                title: format!("{}. {}", section_index + 1, &section.title),
                file_path: None,
            });

            section
                .topics
                .iter()
                .enumerate()
                .for_each(|(topic_index, topic)| {
                    let topic_slug = generate_topic_slug(grammar, section_index, topic_index);
                    let topic_file = generated_folder.join(&topic_slug).join("index.md");

                    entries.push(NavigationEntry {
                        indentation_level: 2,
                        title: format!(
                            "{}.{}. {}",
                            section_index + 1,
                            topic_index + 1,
                            &topic.title
                        ),
                        file_path: Some(topic_file.clone()),
                    });

                    std::fs::create_dir_all(topic_file.parent().unwrap()).unwrap();
                    let mut w = File::create(topic_file).expect("Unable to create file");
                    writeln!(w, "# {}", topic.title).unwrap();
                    writeln!(w).unwrap();
                    writeln!(w, "<!-- markdownlint-disable no-inline-html -->").unwrap();
                    writeln!(w, "<!-- markdownlint-disable no-space-in-emphasis -->").unwrap();
                    writeln!(w, "<!-- cSpell:disable -->").unwrap();

                    match &topic.definition {
                        None => {}
                        Some(definition) => {
                            writeln!(w).unwrap();
                            writeln!(w, "<div class=\"admonition summary\">").unwrap();
                            writeln!(w, "<p class=\"admonition-title\">Grammar</p>").unwrap();

                            grammar
                                .productions
                                .get(definition)
                                .unwrap()
                                .iter()
                                .for_each(|production| {
                                    writeln!(w).unwrap();
                                    write_production(&mut w, production, &context);
                                });

                            writeln!(w).unwrap();
                            writeln!(w, "</div>").unwrap();
                        }
                    }

                    writeln!(w).unwrap();
                    writeln!(
                        &w,
                        "--8<-- \"{}\"",
                        topic.notes.as_ref().unwrap_or(
                            &"specification/notes/under-construction-snippet.md".to_string()
                        )
                    )
                    .unwrap();
                });
        });
}

fn generate_context(grammar: &Grammar) -> SpecProductionContext {
    let context = SpecProductionContext {
        grammar: grammar,
        productions_location: grammar
            .manifest
            .sections
            .iter()
            .enumerate()
            .flat_map(|(section_index, section)| {
                section
                    .topics
                    .iter()
                    .enumerate()
                    .flat_map(move |(topic_index, topic)| {
                        topic.definition.iter().flat_map(move |definition| {
                            grammar.productions.get(definition).unwrap().iter().map(
                                move |production| {
                                    (
                                        production.name.clone(),
                                        format!(
                                            "../../{}",
                                            generate_topic_slug(
                                                grammar,
                                                section_index,
                                                topic_index
                                            )
                                        ),
                                    )
                                },
                            )
                        })
                    })
            })
            .collect(),
    };
    context
}

pub fn generate_topic_slug(grammar: &Grammar, section_index: usize, topic_index: usize) -> String {
    let section = grammar.manifest.sections.get(section_index).unwrap();
    let topic = section.topics.get(topic_index).unwrap();

    return format!(
        "{:0>2}-{}/{:0>2}-{}",
        section_index + 1,
        section.title.to_ascii_lowercase().replace(" ", "-"),
        topic_index + 1,
        topic.title.to_ascii_lowercase().replace(" ", "-"),
    );
}
