use crate::bush::{Bush, BushNode};

use serde::{Deserialize, Serialize};
use serde::ser::{SerializeSeq, SerializeStruct};


impl<T> Serialize for Bush<T>
where
    T: Serialize
{

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where
        S: serde::Serializer
    {
        let mut seq = serializer.serialize_seq(Some(self.top_layer_length()))?;
        for item in self.iter_nodes() {
            seq.serialize_element(item)?;
        }
        seq.end()
    }

}


impl<T> Serialize for BushNode<T>
where 
    T: Serialize
{
    
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where
        S: serde::Serializer
    {
        let mut state = serializer.serialize_struct("BushNode", 2)?;
        state.serialize_field("item", &self.item)?;
        state.serialize_field("children", &self.children)?;
        state.end()
    }
    
}


// impl<'de, T> Deserialize<'de> for Bush<T>
// where
//     T: Deserialize<'a>
// {

//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
//     where
//         D: serde::Deserializer<'de>
//     {
//         let mut bush = Bush::new();
//         let mut seq = <Vec<BushNode<T>>>::deserialize(deserializer)?;
//         for node in seq.drain(..) {
//             bush.add_node(node);
//         }
//         Ok(bush)
//     }

// }


// impl<'de, T> Deserialize<'de> for BushNode<T>
// where
//     T: Deserialize<'de>
// {

//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where 
//         D: serde::Deserializer<'de>
//     {
//         let mut state = deserializer.deserialize_struct("BushNode", &["item", "children"], BushNodeVisitor)?;
//     }

// }

