use crate::flag;

pub trait Asset
{
    fn get_asset_info(&self) -> &AssetInfo;
}

#[derive(Default)]
pub struct AssetInfo
{
    pub id: u32,
    pub flags: AssetFlags
}

impl Asset for AssetInfo
{
    fn get_asset_info(&self) -> &AssetInfo
    {
        self
    }
}

#[derive(Default)]
pub struct AssetFlags
{
    pub enabled: flag::Flag,
    pub persistent: flag::Flag,
    pub cutscene_visible: flag::Flag,
    pub receive_shadows: flag::Flag
}