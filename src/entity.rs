use crate::math;
use crate::flag;
use crate::asset;

#[derive(Default)]
pub struct Entity
{
    pub info: asset::AssetInfo,
    pub transform: Transform,
    pub render_flags: EntityRenderFlags,
    pub collision_flags: EntityCollisionFlags
}

impl asset::Asset for Entity
{
    fn get_asset_info(&self) -> &asset::AssetInfo
    {
        &self.info
    }
}

#[derive(Default)]
pub struct EntityRenderFlags
{
    pub visible: flag::Flag,
    pub stackable: flag::Flag,
    pub disable_shadow: flag::Flag,
}

#[derive(Default)]
pub struct EntityCollisionFlags
{
    pub precise: flag::Flag,
    pub hittable: flag::Flag,
    pub animate: flag::Flag,
    pub grabbable: flag::Flag,
}

#[derive(Default)]
pub struct Transform
{
    pub pos: math::Vec3,
    pub ang: math::Vec3,
    pub scale: math::Vec3
}