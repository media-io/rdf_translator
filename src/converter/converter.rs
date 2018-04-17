
use definition::definition::Definition;
use definition::ebucore::EbuCore;
use definition::owl::Owl;
use definition::rdf::Rdf;
use definition::rdfs::Rdfs;
use definition::undefined::Undefined;
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
    $graph.add_namespace(&Namespace::new($label.to_string(), Uri::new($namespace.to_owned())))
  );
  (node => $graph:expr, $namespace:expr, $label:expr) => (
    $graph.create_uri_node(&Uri::new(Definition::get_label($namespace, $label)))
  )
}

macro_rules! add {
  (definition => $definition:expr, $graph:expr, $subject:expr, $label:expr, $object:block) => ({
    let predicate_namespace = $definition;
    let predicate = build!(node => $graph, &predicate_namespace, $label);
    let object = $object;

    let triple = Triple::new(&$subject, &predicate, &object);
    $graph.add_triple(&triple);
  });
  ($namespace:expr, $graph:expr, $subject:expr, $label:expr, $object:block) => ({
    match $namespace.as_ref().map(|s| &s[..]) {
      Some("ebucore") => {
        add!(definition => EbuCore{}, $graph, $subject, $label, $object)
      },
      Some("owl") => {
        add!(definition => Owl{}, $graph, $subject, $label, $object)
      },
      Some("rdf") => {
        add!(definition => Rdf{}, $graph, $subject, $label, $object)
      },
      Some("rdfs") => {
        add!(definition => Rdfs{}, $graph, $subject, $label, $object)
      },
      Some("xsi") => {
        add!(definition => Xsi{}, $graph, $subject, $label, $object)
      },
      None => {
        add!(definition => Undefined{}, $graph, $subject, $label, $object)
      },
       _ => {
        panic!("unable to process namespace {:?}", $namespace);
      },
    }
  })
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

  pub fn add(&mut self, subject: &Node, namespace: &Option<String>, label: &str, content: &str) {
    add!(namespace, self.graph, subject, label, {
      self.graph.create_literal_node(content.to_string())
    });
  }

  pub fn add_with_language(&mut self, subject: &Node, namespace: &Option<String>, label: &str, content: &str, lang: &str) {
    add!(namespace, self.graph, subject, label, {
      self.graph.create_literal_node_with_language(content.to_string(), lang.to_string())
    });
  }

  pub fn add_with_datatype(&mut self, subject: &Node, namespace: &Option<String>, label: &str, content: &str, datatype: &str) {
    add!(namespace, self.graph, subject, label, {
      self.graph.create_literal_node_with_data_type(content.to_string(), &Uri::new(datatype.to_string()))
    });
  }

  pub fn add_uri(&mut self, subject: &Node, namespace: &Option<String>, label: &str, content: &str) {
    add!(namespace, self.graph, subject, label, {
      self.graph.create_uri_node(&Uri::new(content.to_string()))
    });
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