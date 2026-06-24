# TODO 

## Parse Implementation

It may be helpful to use parser generators: 
1. rust nom 
1. rust pest

### Parser parallel

At the moment all Node are `Ast<Mutex>`, which is the measure for parallelism. 
This seems a overkill.

The node can just be a `Box<Node>`, while paragraph are stored as `Arc<RwLock<Vec<Node>>>`, as we only need parallelism in the paragraph level.

## Compatibilities 

1. Likely not compatible in windows due to file/path name difference. Add compatibility.
