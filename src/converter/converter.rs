
use definition::definition::Definition;
use definition::ebucore::EbuCore;
use definition::owl::Owl;
use definition::rdf::Rdf;
use definition::rdfs::Rdfs;
use definition::xsi::Xsi;

use rdf::graph::Graph;
use rdf::namespace::Namespace;
use rdf::node::Node;
use rdf::triple::Triple;
use rdf::uri::Uri;
use rdf::writer::n_triples_writer::NTriplesWriter;
use rdf::writer::rdf_writer::RdfWriter;
use rdf::writer::turtle_writer::TurtleWriter;

macro_rules! build {
  ($graph:expr, $namespace:expr, $label:expr) => (
    $graph.add_namespace(&Namespace::new($label.to_string(), Uri::new($namespace.to_owned())));
  );
  (node => $graph:expr, $namespace:expr, $label:expr) => (
    $graph.create_uri_node(&Uri::new(Definition::get_label(&$namespace, $label)))
  )
}

#[derive(Debug)]
pub struct Converter {
  pub graph: Graph
}

impl Converter {
  pub fn new() -> Converter {
    let ebucore = EbuCore{};
    let owl = Owl{};
    let rdf = Rdf{};
    let rdfs = Rdfs{};
    let xsi = Xsi{};

    let mut graph = Graph::new(None);
    build!(graph, Definition::get_namespace(&rdf), "rdf");
    build!(graph, Definition::get_namespace(&rdfs), "rdfs");
    build!(graph, Definition::get_namespace(&owl), "owl");
    build!(graph, Definition::get_namespace(&ebucore), "ebucore");
    build!(graph, Definition::get_namespace(&xsi), "xsi");
    build!(graph, "urn:ebu:metadata-schema:ebuCore_2012", "default");

    Converter {
      graph: graph
    }
  }

  pub fn create_subject(&mut self, value: &str) -> Node {
    let subject = self.graph.create_uri_node(&Uri::new(value.to_string()));
    subject
  }

  pub fn add(&mut self, subject: &Node, namespace: &str, label: &str, content: String) {
    let predicate_namespace =
      match namespace {
        "ebucore" => EbuCore{},
         _ => {
          panic!("unable to process namespace {:?}", namespace);
        },
      };

    let predicate = build!(node => self.graph, predicate_namespace, label);
    let object = self.graph.create_literal_node(content);

    let triple = Triple::new(&subject, &predicate, &object);
    self.graph.add_triple(&triple);
  }

  pub fn add_with_language(&mut self, subject: &Node, namespace: &str, label: &str, content: String, lang: &str) {
    let predicate_namespace =
      match namespace {
        "ebucore" => EbuCore{},
         _ => {
          panic!("unable to process namespace {:?}", namespace);
        },
      };

    let predicate = build!(node => self.graph, predicate_namespace, label);
    let object = self.graph.create_literal_node_with_language(content, lang.to_string());

    let triple = Triple::new(&subject, &predicate, &object);
    self.graph.add_triple(&triple);
  }

  pub fn add_with_datatype(&mut self, subject: &Node, namespace: &str, label: &str, content: String, datatype: &str) {
    let predicate_namespace =
      match namespace {
        "ebucore" => EbuCore{},
         _ => {
          panic!("unable to process namespace {:?}", namespace);
        },
      };

    let predicate = build!(node => self.graph, predicate_namespace, label);
    let object = self.graph.create_literal_node_with_data_type(content, &Uri::new(datatype.to_string()));

    let triple = Triple::new(&subject, &predicate, &object);
    self.graph.add_triple(&triple);
  }

  pub fn to_ntriple_string(&self) -> String {
    let writer = NTriplesWriter::new();
    writer.write_to_string(&self.graph).unwrap()
  }

  pub fn to_turtle_string(&self) -> String {
    let writer = TurtleWriter::new(self.graph.namespaces());
    writer.write_to_string(&self.graph).unwrap()
  }
}