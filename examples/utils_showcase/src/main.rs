use ezgfx::*;

#[vertex]
struct MyVertex
{
    pos: [f32; 3],
    tex: [f32; 2]
}

fn main()
{
    let vert = MyVertex { pos: [7.0, 2.0, 1.0], tex: [0.0, 1.0] };

    println!("vertex: {:?}", vert);
}
