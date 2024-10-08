use bevy::prelude::*;
use crate::plugins::timeline::resources::TimelineAssets;
use crate::resources::{ChronoSphere, Fonts, Icons};
use super::layout_bundles::{
  create_app_grid_bundle,
  create_empty_grid_area,
  create_topbar_grid_area,
  create_timeline_header_grid_area,
  create_timeline_body_grid_area,
  create_side_panel_grid_area,
};
use super::topbar::spawn_topbar_contents;
use super::header::spawn_header_contents;
use super::timeline::spawn_timeline_body_contents;

pub fn spawn_ui(mut cmd: Commands, timeline_assets: Res<TimelineAssets>, fonts: Res<Fonts>, icons: Res<Icons>, chrono: Res<ChronoSphere>) {
  // App container
  let all_father = cmd.spawn(create_app_grid_bundle()).id();
  // Col 1 row 1
  let empty_row = cmd.spawn(create_empty_grid_area()).id();
  // Col 1 row 2
  let topbar = cmd.spawn(create_topbar_grid_area()).id();
  let topbar_contents = spawn_topbar_contents(&mut cmd, &fonts, &icons); // todo: decouple system
  cmd.entity(topbar).push_children(&[topbar_contents]);
  // Col 1 row 3
  let timeline_header = cmd.spawn(create_timeline_header_grid_area()).id();
  let header_contents = spawn_header_contents(&mut cmd, &fonts);
  cmd.entity(timeline_header).push_children(&[header_contents]);
  // Col 1 row 4
  let timeline_body = cmd.spawn(create_timeline_body_grid_area()).id();
  let timeline_body_contents = spawn_timeline_body_contents(&mut cmd, timeline_assets, &fonts, chrono); // todo: decouple system
  cmd.entity(timeline_body).push_children(&[timeline_body_contents]);

  // Col 2 row 1-4
  let side_modal = cmd.spawn(create_side_panel_grid_area()).id();

  cmd.entity(all_father).push_children(&[empty_row, topbar, timeline_header, timeline_body, side_modal]);
}
