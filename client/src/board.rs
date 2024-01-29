use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::Layout;

pub(super) fn build_board(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    layout: &Layout,
) {
    let (board, mut board_translation) = layout.board_area;

    let square_size_x = board.x / 8.0;
    let square_size_y = board.y / 8.0;
    let square = Vec2::new(square_size_x, square_size_y);

    board_translation.x -= board.x / 2. - square.x / 2.;
    board_translation.y -= board.y / 2. - square.y / 2.;

    for x in 0..8 {
        for y in 0..8 {
            let mut translation = board_translation.clone();
            translation.x += x as f32 * square_size_x;
            translation.y += y as f32 * square_size_y;
            let color = if (x + y) % 2 == 0 { Color::rgb_linear(0.1, 0.1, 0.1) } else { Color::ANTIQUE_WHITE };
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(square).into())
                    .into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(translation),
                ..default()
            });
        }
    }
}
