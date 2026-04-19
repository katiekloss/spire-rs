use std::{collections::HashMap, fs::File, io::Write};

use spire_rs::{Run, cards::{CardInstance, library::Card}, core::Encounter, mcts::{Action, ActionNode, Search}, monsters::{Enemy, Monsters}};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::{
    cmd::Format,
    exec, printer::PrinterContext,
};

fn start_encounter() -> Encounter {
    let mut run = Run {
        floor: 0,
        relics: HashMap::new(),
        health: 70,
        gold: 99,
        max_health: 70,
        deck: vec![],
    };

    for _ in 0..5 {
        run.deck.push(CardInstance::new(Card::SilentStrike));
        run.deck.push(CardInstance::new(Card::SilentDefend));
    }
    run.deck.push(CardInstance::new(Card::Survivor));
    run.deck.push(CardInstance::new(Card::Neutralize));

    let mut encounter = Encounter::new(run);
    encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));

    encounter
}

fn main() -> std::io::Result<()> {
    let mut encounter = start_encounter();
    encounter.begin_turn();

    let mut search = Search::new();

    search.nodes.insert(0, ActionNode {
        id: 0,
        up: None,
        down: vec![],
        action: None,
        position: Search::get_position(&encounter),
        encounter: encounter,
        visited: false,
        expanded: false,
        wins: 0,
        evals: 0,
        uct: 0.
    });

    for _ in 1..100_000 {
        search.next(0);
    }

    let mut graph = graph!(di id!("mcts"));
    graph.add_stmt(Stmt::GAttribute(graphviz_rust::dot_structures::GraphAttributes::Graph(vec![Attribute {0: id!("rankdir"), 1: id!("LR")}])));
    for node in search.nodes.values() {
        let mut node_g = node!(node.id.to_string());
        let text = match node.action {
            Some(Action::PlayAgainst(card)) | Some(Action::PlaySelf(card)) => format!("{:?}", card),
            Some(Action::NextTurn) => "yield".to_string(),
            None => "".to_string()
        };

        let text = format!("{} {}/{} ({})", text, node.wins, node.evals, node.uct);

        node_g.attributes.push(Attribute {0: id!("label"), 1: Id::Escaped(format!("\"{}\"", text))});
        graph.add_stmt(Stmt::Node(node_g));
        for down in &node.down {
            graph.add_stmt(Stmt::Edge(edge!(node_id!(node.id.to_string()) => node_id!(down.to_string()))));
        }
    }

    {
        File::create("mcts.svg")?.write_all(&exec(graph, &mut PrinterContext::default(), vec![Format::Svg.into()])?)?;
    }

    let mut immediate_children: Vec<(u32, f32)> = search.nodes[&0].down.iter().map(|n| (*n, search.nodes[&n].uct)).collect();
    immediate_children.sort_by(|a, b| b.1.total_cmp(&a.1));
    for node in immediate_children {
        println!("{}", search.nodes[&node.0]);
    }
    Ok(())
}
