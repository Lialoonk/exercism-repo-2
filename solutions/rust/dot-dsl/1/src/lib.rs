pub mod graph{
    use std::collections::HashMap;

    pub mod graph_items{
        pub mod node{
            use std::collections::HashMap;
            #[derive(Clone, PartialEq, Debug)]
            pub struct Node{
                n: String,
                a: HashMap<String, String>,
            }
            impl Node{
                pub fn new(x: &str)->Self{
                    Self { n: x.into(), a: HashMap::new()}
                }
                pub fn with_attrs(mut self, p: &[(&str, &str)])->Self{
                    for(k, v) in p{
                        self.a.insert((*k).into(), (*v).into());
                    }
                    self
                }
                pub fn name(&self) -> &String{
                    &self.n
                }
                pub fn attr(&self, k: &str)->Option<&str>{
                    self.a.get(k).map(String::as_str)
                }
            }
        }

        pub mod edge{
            use std::collections::HashMap;
            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge{
                f: String,
                t: String,
                a: HashMap<String, String>,
            }

            impl Edge{
                pub fn new(f: &str, t: &str) -> Self{
                    Self { f: f.into(), t: t.into(), a: HashMap::new()}
                }
                pub fn with_attrs(mut self, p: &[(&str, &str)]) -> Self{
                    for (k, v) in p{
                        self.a.insert((*k).into(), (*v).into());
                    }
                    self
                }
                pub fn attr(&self, k: &str)->Option<&str>{
                    self.a.get(k).map(String::as_str)
                }
            }
        }
    }

    #[derive(Default)]
    pub struct Graph{
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph{
        pub fn new()->Self{
            Self::default()
        }
        pub fn with_attrs(mut self, p: &[(&str, &str)])->Self{
            for (k, v) in p{
                self.attrs.insert((*k).into(), (*v).into());
            }
            self
        }
        pub fn with_nodes(mut self, v: &[graph_items::node::Node])->Self{
            self.nodes = v.to_vec();
            self
        }
        pub fn with_edges(mut self, v: &[graph_items::edge::Edge]) -> Self{
            self.edges = v.to_vec();
            self
        }
        pub fn node(&self, x: &str)->Option<&graph_items::node::Node>{
            self.nodes.iter().find(|n| n.name() == x)
        }
    }
}
