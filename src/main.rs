mod math;
mod color;
mod entity;
mod flag;
mod asset;

fn main()
{
    let mut ent = entity::Entity::default();

    ent.transform.pos = math::Vec3::new(1.0, 1.0, 1.0);
}