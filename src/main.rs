use three_d::*;

mod cell_block;

use cell_block::Slice;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })?;
    let context = window.gl();


    let slice = Slice::new(10, 10);

    let mut to_render: Vec<Box<dyn Object>> = vec![];
    for cell in slice.into_iter(){
        let color = if cell.value() { Color {
            r: 0,
            g: 0,
            b: 255,
            a: 100,
        } } else { Color {
            r: 255,
            g: 0,
            b: 0,
            a: 100,
        }};

        let mut cube = Gm::new(
            Mesh::new(&context, &CpuMesh::cube()),
            PhysicalMaterial::new_transparent(
                &context,
                &CpuMaterial {
                    albedo: color,
                    ..Default::default()
                },
            ),
        );
        cube.set_transformation(
            Mat4::from_translation(vec3(cell.row(), cell.column(), 1.0)) * Mat4::from_scale(0.2)
        );
        to_render.push(Box::new(cube));
    }

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 2.0, 25.0),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let light0 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, 0.5, 0.5));

    window.render_loop(move |mut frame_input: FrameInput| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                &to_render.iter().map(|o| o.as_ref()).collect::<Vec<&dyn Object>>().as_slice(),
                &[&light0, &light1],
            );

        FrameOutput::default()
    });

    Ok(())
}