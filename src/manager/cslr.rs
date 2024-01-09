#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = "背景配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"背景配置。\",\n  \"required\": [\n    \"background_config\",\n    \"background_type\"\n  ],\n  \"properties\": {\n    \"background_asset_id\": {\n      \"description\": \"背景资产ID。\\n> * 背景是背景图片时,填图片资产ID。\",\n      \"type\": \"string\",\n      \"maxLength\": 64,\n      \"minLength\": 0,\n      \"example\": \"a5d295cdb345c11bd9f36bc22ced3a7b\"\n    },\n    \"background_config\": {\n      \"description\": \"背景文件的URL。\\n> * 通过资产库查询获取,不支持外部URL。\",\n      \"type\": \"string\",\n      \"maxLength\": 2048,\n      \"minLength\": 1,\n      \"example\": \"https://digitalhuman.obs.cn-east-3.myhuaweicloud.com:443/0d697589d98091f12f92c0073501cd79/8cb2f48a2cb006154794741933421100/b2f8a9e9c39b6dc7ed4c0cfd67366c6a.jpg?AccessKeyId=XCQKTA8IWQPCAUQS4SDA&Expires=1671241324&Signature=tD38peKsCkh%2FygE3IffeLbewuEw%3D\"\n    },\n    \"background_cover_url\": {\n      \"description\": \"视频文件封面图片的下载URL。\\n\\n演示素材为视频时有效。\\n> * 分身数字人视频制作此参数不生效。\",\n      \"type\": \"string\",\n      \"maxLength\": 2048,\n      \"minLength\": 0,\n      \"example\": \"https://digitalhuman.obs.cn-east-3.myhuaweicloud.com:443/0d697589d98091f12f92c0073501cd79/8cb2f48a2cb006154794741933421100/4e7f4756d5df4df7b65f398759ae0a52/df0f916bb35e2d12197008bc04da761b.png?AccessKeyId=XCQKTA8IWQPCAUQS4SDA&Expires=1671241324&Signature=tD38peKsCkh%2FygE3IffeLbewuEw%3D\"\n    },\n    \"background_title\": {\n      \"description\": \"背景标题。\\n> * 分身数字人视频制作此参数不生效。\",\n      \"type\": \"string\",\n      \"maxLength\": 256,\n      \"minLength\": 0\n    },\n    \"background_type\": {\n      \"description\": \"背景类型。\\n- IMAGE:图片,用于3D数字人演示素材讲解模式的图片或分身数字背景图片\\n- IMAGE_2D:图片,用于3D数字人主播播报模式的2D场景背景图片\\n- VIDEO:视频\\n- AUDIO:音频\\n> * 分身数字人视频制作仅支持IMAGE\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"IMAGE\",\n        \"IMAGE_2D\",\n        \"VIDEO\",\n        \"AUDIO\"\n      ],\n      \"maxLength\": 2048,\n      \"minLength\": 1\n    },\n    \"human_position_2d\": {\n      \"description\": \"分身数字人在背景图片的位置设置。不设置默认在图片中间。\\n> * 此参数废弃。分身数字人在背景中位置在layer_config参数中配置。\",\n      \"$ref\": \"#/definitions/HumanPosition2D\"\n    },\n    \"human_size_2d\": {\n      \"description\": \"分身数字人在背景图片的大小设置。\\n> * 此参数废弃。分身数字人在背景中大小在layer_config参数中配置。\",\n      \"$ref\": \"#/definitions/HumanSize2D\"\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BackgroundConfigInfo {
    #[doc = "背景资产ID。\n> * 背景是背景图片时,填图片资产ID。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_asset_id: Option<BackgroundConfigInfoBackgroundAssetId>,
    #[doc = "背景文件的URL。\n> * 通过资产库查询获取,不支持外部URL。"]
    pub background_config: BackgroundConfigInfoBackgroundConfig,
    #[doc = "视频文件封面图片的下载URL。\n\n演示素材为视频时有效。\n> * 分身数字人视频制作此参数不生效。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_cover_url: Option<BackgroundConfigInfoBackgroundCoverUrl>,
    #[doc = "背景标题。\n> * 分身数字人视频制作此参数不生效。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_title: Option<BackgroundConfigInfoBackgroundTitle>,
    #[doc = "背景类型。\n- IMAGE:图片,用于3D数字人演示素材讲解模式的图片或分身数字背景图片\n- IMAGE_2D:图片,用于3D数字人主播播报模式的2D场景背景图片\n- VIDEO:视频\n- AUDIO:音频\n> * 分身数字人视频制作仅支持IMAGE"]
    pub background_type: BackgroundConfigInfoBackgroundType,
    #[doc = "分身数字人在背景图片的位置设置。不设置默认在图片中间。\n> * 此参数废弃。分身数字人在背景中位置在layer_config参数中配置。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub human_position_2d: Option<HumanPosition2D>,
    #[doc = "分身数字人在背景图片的大小设置。\n> * 此参数废弃。分身数字人在背景中大小在layer_config参数中配置。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub human_size_2d: Option<HumanSize2D>,
}
impl From<&BackgroundConfigInfo> for BackgroundConfigInfo {
    fn from(value: &BackgroundConfigInfo) -> Self {
        value.clone()
    }
}
impl BackgroundConfigInfo {
    pub fn builder() -> builder::BackgroundConfigInfo {
        Default::default()
    }
}
#[doc = "背景资产ID。\n> * 背景是背景图片时,填图片资产ID。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"背景资产ID。\\n> * 背景是背景图片时,填图片资产ID。\",\n  \"type\": \"string\",\n  \"maxLength\": 64,\n  \"minLength\": 0,\n  \"example\": \"a5d295cdb345c11bd9f36bc22ced3a7b\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BackgroundConfigInfoBackgroundAssetId(String);
impl std::ops::Deref for BackgroundConfigInfoBackgroundAssetId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<BackgroundConfigInfoBackgroundAssetId> for String {
    fn from(value: BackgroundConfigInfoBackgroundAssetId) -> Self {
        value.0
    }
}
impl From<&BackgroundConfigInfoBackgroundAssetId> for BackgroundConfigInfoBackgroundAssetId {
    fn from(value: &BackgroundConfigInfoBackgroundAssetId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for BackgroundConfigInfoBackgroundAssetId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 64usize {
            return Err("longer than 64 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for BackgroundConfigInfoBackgroundAssetId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BackgroundConfigInfoBackgroundAssetId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BackgroundConfigInfoBackgroundAssetId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for BackgroundConfigInfoBackgroundAssetId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "背景文件的URL。\n> * 通过资产库查询获取,不支持外部URL。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"背景文件的URL。\\n> * 通过资产库查询获取,不支持外部URL。\",\n  \"type\": \"string\",\n  \"maxLength\": 2048,\n  \"minLength\": 1,\n  \"example\": \"https://digitalhuman.obs.cn-east-3.myhuaweicloud.com:443/0d697589d98091f12f92c0073501cd79/8cb2f48a2cb006154794741933421100/b2f8a9e9c39b6dc7ed4c0cfd67366c6a.jpg?AccessKeyId=XCQKTA8IWQPCAUQS4SDA&Expires=1671241324&Signature=tD38peKsCkh%2FygE3IffeLbewuEw%3D\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BackgroundConfigInfoBackgroundConfig(String);
impl std::ops::Deref for BackgroundConfigInfoBackgroundConfig {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<BackgroundConfigInfoBackgroundConfig> for String {
    fn from(value: BackgroundConfigInfoBackgroundConfig) -> Self {
        value.0
    }
}
impl From<&BackgroundConfigInfoBackgroundConfig> for BackgroundConfigInfoBackgroundConfig {
    fn from(value: &BackgroundConfigInfoBackgroundConfig) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for BackgroundConfigInfoBackgroundConfig {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 2048usize {
            return Err("longer than 2048 characters");
        }
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for BackgroundConfigInfoBackgroundConfig {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BackgroundConfigInfoBackgroundConfig {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BackgroundConfigInfoBackgroundConfig {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for BackgroundConfigInfoBackgroundConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "视频文件封面图片的下载URL。\n\n演示素材为视频时有效。\n> * 分身数字人视频制作此参数不生效。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"视频文件封面图片的下载URL。\\n\\n演示素材为视频时有效。\\n> * 分身数字人视频制作此参数不生效。\",\n  \"type\": \"string\",\n  \"maxLength\": 2048,\n  \"minLength\": 0,\n  \"example\": \"https://digitalhuman.obs.cn-east-3.myhuaweicloud.com:443/0d697589d98091f12f92c0073501cd79/8cb2f48a2cb006154794741933421100/4e7f4756d5df4df7b65f398759ae0a52/df0f916bb35e2d12197008bc04da761b.png?AccessKeyId=XCQKTA8IWQPCAUQS4SDA&Expires=1671241324&Signature=tD38peKsCkh%2FygE3IffeLbewuEw%3D\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BackgroundConfigInfoBackgroundCoverUrl(String);
impl std::ops::Deref for BackgroundConfigInfoBackgroundCoverUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<BackgroundConfigInfoBackgroundCoverUrl> for String {
    fn from(value: BackgroundConfigInfoBackgroundCoverUrl) -> Self {
        value.0
    }
}
impl From<&BackgroundConfigInfoBackgroundCoverUrl> for BackgroundConfigInfoBackgroundCoverUrl {
    fn from(value: &BackgroundConfigInfoBackgroundCoverUrl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for BackgroundConfigInfoBackgroundCoverUrl {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 2048usize {
            return Err("longer than 2048 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for BackgroundConfigInfoBackgroundCoverUrl {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BackgroundConfigInfoBackgroundCoverUrl {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BackgroundConfigInfoBackgroundCoverUrl {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for BackgroundConfigInfoBackgroundCoverUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "背景标题。\n> * 分身数字人视频制作此参数不生效。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"背景标题。\\n> * 分身数字人视频制作此参数不生效。\",\n  \"type\": \"string\",\n  \"maxLength\": 256,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BackgroundConfigInfoBackgroundTitle(String);
impl std::ops::Deref for BackgroundConfigInfoBackgroundTitle {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<BackgroundConfigInfoBackgroundTitle> for String {
    fn from(value: BackgroundConfigInfoBackgroundTitle) -> Self {
        value.0
    }
}
impl From<&BackgroundConfigInfoBackgroundTitle> for BackgroundConfigInfoBackgroundTitle {
    fn from(value: &BackgroundConfigInfoBackgroundTitle) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for BackgroundConfigInfoBackgroundTitle {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 256usize {
            return Err("longer than 256 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for BackgroundConfigInfoBackgroundTitle {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BackgroundConfigInfoBackgroundTitle {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BackgroundConfigInfoBackgroundTitle {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for BackgroundConfigInfoBackgroundTitle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "背景类型。\n- IMAGE:图片,用于3D数字人演示素材讲解模式的图片或分身数字背景图片\n- IMAGE_2D:图片,用于3D数字人主播播报模式的2D场景背景图片\n- VIDEO:视频\n- AUDIO:音频\n> * 分身数字人视频制作仅支持IMAGE"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"背景类型。\\n- IMAGE:图片,用于3D数字人演示素材讲解模式的图片或分身数字背景图片\\n- IMAGE_2D:图片,用于3D数字人主播播报模式的2D场景背景图片\\n- VIDEO:视频\\n- AUDIO:音频\\n> * 分身数字人视频制作仅支持IMAGE\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"IMAGE\",\n    \"IMAGE_2D\",\n    \"VIDEO\",\n    \"AUDIO\"\n  ],\n  \"maxLength\": 2048,\n  \"minLength\": 1\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BackgroundConfigInfoBackgroundType {
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "IMAGE_2D")]
    Image2d,
    #[serde(rename = "VIDEO")]
    Video,
    #[serde(rename = "AUDIO")]
    Audio,
}
impl From<&BackgroundConfigInfoBackgroundType> for BackgroundConfigInfoBackgroundType {
    fn from(value: &BackgroundConfigInfoBackgroundType) -> Self {
        value.clone()
    }
}
impl ToString for BackgroundConfigInfoBackgroundType {
    fn to_string(&self) -> String {
        match *self {
            Self::Image => "IMAGE".to_string(),
            Self::Image2d => "IMAGE_2D".to_string(),
            Self::Video => "VIDEO".to_string(),
            Self::Audio => "AUDIO".to_string(),
        }
    }
}
impl std::str::FromStr for BackgroundConfigInfoBackgroundType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "IMAGE" => Ok(Self::Image),
            "IMAGE_2D" => Ok(Self::Image2d),
            "VIDEO" => Ok(Self::Video),
            "AUDIO" => Ok(Self::Audio),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for BackgroundConfigInfoBackgroundType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BackgroundConfigInfoBackgroundType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BackgroundConfigInfoBackgroundType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "创建直播间配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"创建直播间配置。\",\n  \"type\": \"object\",\n  \"required\": [\n    \"room_name\"\n  ],\n  \"properties\": {\n    \"backup_model_asset_ids\": {\n      \"description\": \"主播轮换时备选主播数字人资产ID(仅形象资产,不包含音色)。\",\n      \"type\": \"array\",\n      \"items\": {\n        \"type\": \"string\",\n        \"maxLength\": 32,\n        \"minLength\": 0\n      },\n      \"maxItems\": 32,\n      \"minItems\": 0\n    },\n    \"interaction_rules\": {\n      \"description\": \"互动规则列表\",\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/definitions/InteractionRuleInfo\"\n      },\n      \"maxItems\": 100,\n      \"minItems\": 0\n    },\n    \"live_event_callback_config\": {\n      \"$ref\": \"#/definitions/LiveEventCallBackConfig\"\n    },\n    \"output_urls\": {\n      \"description\": \"RTMP视频推流第三方直播平台地址。\",\n      \"type\": \"array\",\n      \"items\": {\n        \"type\": \"string\",\n        \"maxLength\": 1024,\n        \"minLength\": 0\n      },\n      \"maxItems\": 1,\n      \"minItems\": 0\n    },\n    \"play_policy\": {\n      \"description\": \"剧本播放策略\",\n      \"$ref\": \"#/definitions/PlayPolicy\"\n    },\n    \"review_config\": {\n      \"description\": \"内容审核配置\",\n      \"$ref\": \"#/definitions/ReviewConfig\"\n    },\n    \"room_description\": {\n      \"description\": \"直播间描述。\",\n      \"type\": \"string\",\n      \"maxLength\": 1024,\n      \"minLength\": 0,\n      \"example\": \"课件\"\n    },\n    \"room_name\": {\n      \"description\": \"直播间名称\",\n      \"type\": \"string\",\n      \"maxLength\": 256,\n      \"minLength\": 1,\n      \"example\": \"大自然的传说\"\n    },\n    \"room_type\": {\n      \"description\": \"直播间类型。\\n* NORMAL: 普通直播间,直播间一直存在,可以反复开播\\n* TEMP: 临时直播间,直播任务结束后自动清理直播间。\\n* TEMPLATE: 直播间模板。\",\n      \"default\": \"NORMAL\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"NORMAL\",\n        \"TEMP\",\n        \"TEMPLATE\"\n      ],\n      \"maxLength\": 16,\n      \"minLength\": 0\n    },\n    \"scene_scripts\": {\n      \"description\": \"默认直播剧本列表。\",\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/definitions/LiveVideoScriptInfo\"\n      },\n      \"maxItems\": 100,\n      \"minItems\": 1\n    },\n    \"shared_config\": {\n      \"description\": \"共享配置\",\n      \"$ref\": \"#/definitions/SharedConfig\"\n    },\n    \"stream_keys\": {\n      \"description\": \"RTMP视频推流第三方直播平台流秘钥,与推流地址对应。\",\n      \"type\": \"array\",\n      \"items\": {\n        \"type\": \"string\",\n        \"maxLength\": 1024,\n        \"minLength\": 0\n      },\n      \"maxItems\": 1,\n      \"minItems\": 0\n    },\n    \"video_config\": {\n      \"description\": \"视频输出配置。\",\n      \"$ref\": \"#/definitions/VideoConfig\"\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSmartLiveRoomReq {
    #[doc = "主播轮换时备选主播数字人资产ID(仅形象资产,不包含音色)。"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub backup_model_asset_ids: Vec<CreateSmartLiveRoomReqBackupModelAssetIdsItem>,
    #[doc = "互动规则列表"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction_rules: Vec<InteractionRuleInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_event_callback_config: Option<LiveEventCallBackConfig>,
    #[doc = "RTMP视频推流第三方直播平台地址。"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_urls: Vec<CreateSmartLiveRoomReqOutputUrlsItem>,
    #[doc = "剧本播放策略"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub play_policy: Option<PlayPolicy>,
    #[doc = "内容审核配置"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_config: Option<ReviewConfig>,
    #[doc = "直播间描述。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_description: Option<CreateSmartLiveRoomReqRoomDescription>,
    #[doc = "直播间名称"]
    pub room_name: CreateSmartLiveRoomReqRoomName,
    #[doc = "直播间类型。\n* NORMAL: 普通直播间,直播间一直存在,可以反复开播\n* TEMP: 临时直播间,直播任务结束后自动清理直播间。\n* TEMPLATE: 直播间模板。"]
    #[serde(default = "defaults::create_smart_live_room_req_room_type")]
    pub room_type: CreateSmartLiveRoomReqRoomType,
    #[doc = "默认直播剧本列表。"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scene_scripts: Vec<LiveVideoScriptInfo>,
    #[doc = "共享配置"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_config: Option<SharedConfig>,
    #[doc = "RTMP视频推流第三方直播平台流秘钥,与推流地址对应。"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stream_keys: Vec<CreateSmartLiveRoomReqStreamKeysItem>,
    #[doc = "视频输出配置。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_config: Option<VideoConfig>,
}
impl From<&CreateSmartLiveRoomReq> for CreateSmartLiveRoomReq {
    fn from(value: &CreateSmartLiveRoomReq) -> Self {
        value.clone()
    }
}
impl CreateSmartLiveRoomReq {
    pub fn builder() -> builder::CreateSmartLiveRoomReq {
        Default::default()
    }
}
#[doc = "CreateSmartLiveRoomReqBackupModelAssetIdsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"maxLength\": 32,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreateSmartLiveRoomReqBackupModelAssetIdsItem(String);
impl std::ops::Deref for CreateSmartLiveRoomReqBackupModelAssetIdsItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<CreateSmartLiveRoomReqBackupModelAssetIdsItem> for String {
    fn from(value: CreateSmartLiveRoomReqBackupModelAssetIdsItem) -> Self {
        value.0
    }
}
impl From<&CreateSmartLiveRoomReqBackupModelAssetIdsItem>
    for CreateSmartLiveRoomReqBackupModelAssetIdsItem
{
    fn from(value: &CreateSmartLiveRoomReqBackupModelAssetIdsItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CreateSmartLiveRoomReqBackupModelAssetIdsItem {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 32usize {
            return Err("longer than 32 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for CreateSmartLiveRoomReqBackupModelAssetIdsItem {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CreateSmartLiveRoomReqBackupModelAssetIdsItem {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CreateSmartLiveRoomReqBackupModelAssetIdsItem {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSmartLiveRoomReqBackupModelAssetIdsItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "CreateSmartLiveRoomReqOutputUrlsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"maxLength\": 1024,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreateSmartLiveRoomReqOutputUrlsItem(String);
impl std::ops::Deref for CreateSmartLiveRoomReqOutputUrlsItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<CreateSmartLiveRoomReqOutputUrlsItem> for String {
    fn from(value: CreateSmartLiveRoomReqOutputUrlsItem) -> Self {
        value.0
    }
}
impl From<&CreateSmartLiveRoomReqOutputUrlsItem> for CreateSmartLiveRoomReqOutputUrlsItem {
    fn from(value: &CreateSmartLiveRoomReqOutputUrlsItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CreateSmartLiveRoomReqOutputUrlsItem {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 1024usize {
            return Err("longer than 1024 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for CreateSmartLiveRoomReqOutputUrlsItem {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CreateSmartLiveRoomReqOutputUrlsItem {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CreateSmartLiveRoomReqOutputUrlsItem {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSmartLiveRoomReqOutputUrlsItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "直播间描述。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"直播间描述。\",\n  \"type\": \"string\",\n  \"maxLength\": 1024,\n  \"minLength\": 0,\n  \"example\": \"课件\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreateSmartLiveRoomReqRoomDescription(String);
impl std::ops::Deref for CreateSmartLiveRoomReqRoomDescription {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<CreateSmartLiveRoomReqRoomDescription> for String {
    fn from(value: CreateSmartLiveRoomReqRoomDescription) -> Self {
        value.0
    }
}
impl From<&CreateSmartLiveRoomReqRoomDescription> for CreateSmartLiveRoomReqRoomDescription {
    fn from(value: &CreateSmartLiveRoomReqRoomDescription) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CreateSmartLiveRoomReqRoomDescription {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 1024usize {
            return Err("longer than 1024 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for CreateSmartLiveRoomReqRoomDescription {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CreateSmartLiveRoomReqRoomDescription {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CreateSmartLiveRoomReqRoomDescription {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSmartLiveRoomReqRoomDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "直播间名称"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"直播间名称\",\n  \"type\": \"string\",\n  \"maxLength\": 256,\n  \"minLength\": 1,\n  \"example\": \"大自然的传说\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreateSmartLiveRoomReqRoomName(String);
impl std::ops::Deref for CreateSmartLiveRoomReqRoomName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<CreateSmartLiveRoomReqRoomName> for String {
    fn from(value: CreateSmartLiveRoomReqRoomName) -> Self {
        value.0
    }
}
impl From<&CreateSmartLiveRoomReqRoomName> for CreateSmartLiveRoomReqRoomName {
    fn from(value: &CreateSmartLiveRoomReqRoomName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CreateSmartLiveRoomReqRoomName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 256usize {
            return Err("longer than 256 characters");
        }
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for CreateSmartLiveRoomReqRoomName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CreateSmartLiveRoomReqRoomName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CreateSmartLiveRoomReqRoomName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSmartLiveRoomReqRoomName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "直播间类型。\n* NORMAL: 普通直播间,直播间一直存在,可以反复开播\n* TEMP: 临时直播间,直播任务结束后自动清理直播间。\n* TEMPLATE: 直播间模板。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"直播间类型。\\n* NORMAL: 普通直播间,直播间一直存在,可以反复开播\\n* TEMP: 临时直播间,直播任务结束后自动清理直播间。\\n* TEMPLATE: 直播间模板。\",\n  \"default\": \"NORMAL\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"NORMAL\",\n    \"TEMP\",\n    \"TEMPLATE\"\n  ],\n  \"maxLength\": 16,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum CreateSmartLiveRoomReqRoomType {
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "TEMP")]
    Temp,
    #[serde(rename = "TEMPLATE")]
    Template,
}
impl From<&CreateSmartLiveRoomReqRoomType> for CreateSmartLiveRoomReqRoomType {
    fn from(value: &CreateSmartLiveRoomReqRoomType) -> Self {
        value.clone()
    }
}
impl ToString for CreateSmartLiveRoomReqRoomType {
    fn to_string(&self) -> String {
        match *self {
            Self::Normal => "NORMAL".to_string(),
            Self::Temp => "TEMP".to_string(),
            Self::Template => "TEMPLATE".to_string(),
        }
    }
}
impl std::str::FromStr for CreateSmartLiveRoomReqRoomType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "NORMAL" => Ok(Self::Normal),
            "TEMP" => Ok(Self::Temp),
            "TEMPLATE" => Ok(Self::Template),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for CreateSmartLiveRoomReqRoomType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CreateSmartLiveRoomReqRoomType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CreateSmartLiveRoomReqRoomType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for CreateSmartLiveRoomReqRoomType {
    fn default() -> Self {
        CreateSmartLiveRoomReqRoomType::Normal
    }
}
#[doc = "CreateSmartLiveRoomReqStreamKeysItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"maxLength\": 1024,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreateSmartLiveRoomReqStreamKeysItem(String);
impl std::ops::Deref for CreateSmartLiveRoomReqStreamKeysItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<CreateSmartLiveRoomReqStreamKeysItem> for String {
    fn from(value: CreateSmartLiveRoomReqStreamKeysItem) -> Self {
        value.0
    }
}
impl From<&CreateSmartLiveRoomReqStreamKeysItem> for CreateSmartLiveRoomReqStreamKeysItem {
    fn from(value: &CreateSmartLiveRoomReqStreamKeysItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CreateSmartLiveRoomReqStreamKeysItem {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 1024usize {
            return Err("longer than 1024 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for CreateSmartLiveRoomReqStreamKeysItem {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CreateSmartLiveRoomReqStreamKeysItem {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CreateSmartLiveRoomReqStreamKeysItem {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSmartLiveRoomReqStreamKeysItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "创建直播间响应。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"创建直播间响应。\",\n  \"properties\": {\n    \"room_id\": {\n      \"description\": \"直播间ID\",\n      \"type\": \"string\",\n      \"maxLength\": 64,\n      \"minLength\": 0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSmartLiveRoomRsp {
    #[doc = "直播间ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<CreateSmartLiveRoomRspRoomId>,
}
impl From<&CreateSmartLiveRoomRsp> for CreateSmartLiveRoomRsp {
    fn from(value: &CreateSmartLiveRoomRsp) -> Self {
        value.clone()
    }
}
impl CreateSmartLiveRoomRsp {
    pub fn builder() -> builder::CreateSmartLiveRoomRsp {
        Default::default()
    }
}
#[doc = "直播间ID"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"直播间ID\",\n  \"type\": \"string\",\n  \"maxLength\": 64,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CreateSmartLiveRoomRspRoomId(String);
impl std::ops::Deref for CreateSmartLiveRoomRspRoomId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<CreateSmartLiveRoomRspRoomId> for String {
    fn from(value: CreateSmartLiveRoomRspRoomId) -> Self {
        value.0
    }
}
impl From<&CreateSmartLiveRoomRspRoomId> for CreateSmartLiveRoomRspRoomId {
    fn from(value: &CreateSmartLiveRoomRspRoomId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for CreateSmartLiveRoomRspRoomId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 64usize {
            return Err("longer than 64 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for CreateSmartLiveRoomRspRoomId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CreateSmartLiveRoomRspRoomId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CreateSmartLiveRoomRspRoomId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSmartLiveRoomRspRoomId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "命中条件配置"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"命中条件配置\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"priority\": {\n      \"description\": \"优先级,数值越低优先级越高;取值0-999,默认值为500,为可选值\",\n      \"type\": \"integer\",\n      \"maximum\": 999.0,\n      \"minimum\": 0.0\n    },\n    \"relation\": {\n      \"description\": \"条件关系;取值And或者Or\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"AND\",\n        \"OR\"\n      ],\n      \"maxLength\": 16,\n      \"minLength\": 0\n    },\n    \"tags\": {\n      \"description\": \"匹配关系配置\",\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/definitions/HitConditionTag\"\n      },\n      \"maxItems\": 5,\n      \"minItems\": 1\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HitCondition {
    #[doc = "优先级,数值越低优先级越高;取值0-999,默认值为500,为可选值"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[doc = "条件关系;取值And或者Or"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<HitConditionRelation>,
    #[doc = "匹配关系配置"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<HitConditionTag>,
}
impl From<&HitCondition> for HitCondition {
    fn from(value: &HitCondition) -> Self {
        value.clone()
    }
}
impl HitCondition {
    pub fn builder() -> builder::HitCondition {
        Default::default()
    }
}
#[doc = "条件关系;取值And或者Or"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"条件关系;取值And或者Or\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"AND\",\n    \"OR\"\n  ],\n  \"maxLength\": 16,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum HitConditionRelation {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}
impl From<&HitConditionRelation> for HitConditionRelation {
    fn from(value: &HitConditionRelation) -> Self {
        value.clone()
    }
}
impl ToString for HitConditionRelation {
    fn to_string(&self) -> String {
        match *self {
            Self::And => "AND".to_string(),
            Self::Or => "OR".to_string(),
        }
    }
}
impl std::str::FromStr for HitConditionRelation {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for HitConditionRelation {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for HitConditionRelation {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for HitConditionRelation {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "命中条件定义"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"命中条件定义\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"match\": {\n      \"description\": \"匹配类型\\n- EQUAL: 完全相等\\n- REGEX:正则匹配\\n- MATH_GT:数值大于\\n- MATH_GE: 数值大于等于\\n- MATH_LT:数值小于\\n- MATH_LE:数值小于等于\\n- MATH_EQ:数值相等\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"EQUAL\",\n        \"REGEX\",\n        \"MATH_GT\",\n        \"MATH_GE\",\n        \"MATH_LT\",\n        \"MATH_LE\",\n        \"MATH_EQ\"\n      ],\n      \"maxLength\": 32,\n      \"minLength\": 0\n    },\n    \"operation\": {\n      \"description\": \"字段处理\\n- SUM: 累计\\n- AVG:平均\\n- COUNT: 计数\\n- NONE: 无处理\",\n      \"default\": \"NONE\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"SUM\",\n        \"AVG\",\n        \"COUNT\",\n        \"NONE\"\n      ],\n      \"maxLength\": 32,\n      \"minLength\": 0\n    },\n    \"tag\": {\n      \"description\": \"事件内容关键字段\",\n      \"type\": \"string\",\n      \"maxLength\": 256,\n      \"minLength\": 0\n    },\n    \"value\": {\n      \"description\": \"匹配值\",\n      \"type\": \"string\",\n      \"maxLength\": 1024,\n      \"minLength\": 0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HitConditionTag {
    #[doc = "匹配类型\n- EQUAL: 完全相等\n- REGEX:正则匹配\n- MATH_GT:数值大于\n- MATH_GE: 数值大于等于\n- MATH_LT:数值小于\n- MATH_LE:数值小于等于\n- MATH_EQ:数值相等"]
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<HitConditionTagMatch>,
    #[doc = "字段处理\n- SUM: 累计\n- AVG:平均\n- COUNT: 计数\n- NONE: 无处理"]
    #[serde(default = "defaults::hit_condition_tag_operation")]
    pub operation: HitConditionTagOperation,
    #[doc = "事件内容关键字段"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<HitConditionTagTag>,
    #[doc = "匹配值"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<HitConditionTagValue>,
}
impl From<&HitConditionTag> for HitConditionTag {
    fn from(value: &HitConditionTag) -> Self {
        value.clone()
    }
}
impl HitConditionTag {
    pub fn builder() -> builder::HitConditionTag {
        Default::default()
    }
}
#[doc = "匹配类型\n- EQUAL: 完全相等\n- REGEX:正则匹配\n- MATH_GT:数值大于\n- MATH_GE: 数值大于等于\n- MATH_LT:数值小于\n- MATH_LE:数值小于等于\n- MATH_EQ:数值相等"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"匹配类型\\n- EQUAL: 完全相等\\n- REGEX:正则匹配\\n- MATH_GT:数值大于\\n- MATH_GE: 数值大于等于\\n- MATH_LT:数值小于\\n- MATH_LE:数值小于等于\\n- MATH_EQ:数值相等\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"EQUAL\",\n    \"REGEX\",\n    \"MATH_GT\",\n    \"MATH_GE\",\n    \"MATH_LT\",\n    \"MATH_LE\",\n    \"MATH_EQ\"\n  ],\n  \"maxLength\": 32,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum HitConditionTagMatch {
    #[serde(rename = "EQUAL")]
    Equal,
    #[serde(rename = "REGEX")]
    Regex,
    #[serde(rename = "MATH_GT")]
    MathGt,
    #[serde(rename = "MATH_GE")]
    MathGe,
    #[serde(rename = "MATH_LT")]
    MathLt,
    #[serde(rename = "MATH_LE")]
    MathLe,
    #[serde(rename = "MATH_EQ")]
    MathEq,
}
impl From<&HitConditionTagMatch> for HitConditionTagMatch {
    fn from(value: &HitConditionTagMatch) -> Self {
        value.clone()
    }
}
impl ToString for HitConditionTagMatch {
    fn to_string(&self) -> String {
        match *self {
            Self::Equal => "EQUAL".to_string(),
            Self::Regex => "REGEX".to_string(),
            Self::MathGt => "MATH_GT".to_string(),
            Self::MathGe => "MATH_GE".to_string(),
            Self::MathLt => "MATH_LT".to_string(),
            Self::MathLe => "MATH_LE".to_string(),
            Self::MathEq => "MATH_EQ".to_string(),
        }
    }
}
impl std::str::FromStr for HitConditionTagMatch {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "EQUAL" => Ok(Self::Equal),
            "REGEX" => Ok(Self::Regex),
            "MATH_GT" => Ok(Self::MathGt),
            "MATH_GE" => Ok(Self::MathGe),
            "MATH_LT" => Ok(Self::MathLt),
            "MATH_LE" => Ok(Self::MathLe),
            "MATH_EQ" => Ok(Self::MathEq),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for HitConditionTagMatch {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for HitConditionTagMatch {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for HitConditionTagMatch {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "字段处理\n- SUM: 累计\n- AVG:平均\n- COUNT: 计数\n- NONE: 无处理"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"字段处理\\n- SUM: 累计\\n- AVG:平均\\n- COUNT: 计数\\n- NONE: 无处理\",\n  \"default\": \"NONE\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"SUM\",\n    \"AVG\",\n    \"COUNT\",\n    \"NONE\"\n  ],\n  \"maxLength\": 32,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum HitConditionTagOperation {
    #[serde(rename = "SUM")]
    Sum,
    #[serde(rename = "AVG")]
    Avg,
    #[serde(rename = "COUNT")]
    Count,
    #[serde(rename = "NONE")]
    None,
}
impl From<&HitConditionTagOperation> for HitConditionTagOperation {
    fn from(value: &HitConditionTagOperation) -> Self {
        value.clone()
    }
}
impl ToString for HitConditionTagOperation {
    fn to_string(&self) -> String {
        match *self {
            Self::Sum => "SUM".to_string(),
            Self::Avg => "AVG".to_string(),
            Self::Count => "COUNT".to_string(),
            Self::None => "NONE".to_string(),
        }
    }
}
impl std::str::FromStr for HitConditionTagOperation {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "SUM" => Ok(Self::Sum),
            "AVG" => Ok(Self::Avg),
            "COUNT" => Ok(Self::Count),
            "NONE" => Ok(Self::None),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for HitConditionTagOperation {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for HitConditionTagOperation {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for HitConditionTagOperation {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for HitConditionTagOperation {
    fn default() -> Self {
        HitConditionTagOperation::None
    }
}
#[doc = "事件内容关键字段"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"事件内容关键字段\",\n  \"type\": \"string\",\n  \"maxLength\": 256,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct HitConditionTagTag(String);
impl std::ops::Deref for HitConditionTagTag {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<HitConditionTagTag> for String {
    fn from(value: HitConditionTagTag) -> Self {
        value.0
    }
}
impl From<&HitConditionTagTag> for HitConditionTagTag {
    fn from(value: &HitConditionTagTag) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for HitConditionTagTag {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 256usize {
            return Err("longer than 256 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for HitConditionTagTag {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for HitConditionTagTag {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for HitConditionTagTag {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for HitConditionTagTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "匹配值"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"匹配值\",\n  \"type\": \"string\",\n  \"maxLength\": 1024,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct HitConditionTagValue(String);
impl std::ops::Deref for HitConditionTagValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<HitConditionTagValue> for String {
    fn from(value: HitConditionTagValue) -> Self {
        value.0
    }
}
impl From<&HitConditionTagValue> for HitConditionTagValue {
    fn from(value: &HitConditionTagValue) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for HitConditionTagValue {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 1024usize {
            return Err("longer than 1024 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for HitConditionTagValue {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for HitConditionTagValue {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for HitConditionTagValue {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for HitConditionTagValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "分身数字人在背景图片位置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"分身数字人在背景图片位置。\",\n  \"properties\": {\n    \"position\": {\n      \"description\": \"分身数字人在背景图片中的位置。\\n* LEFT: 左\\n* MIDDLE: 中\\n* RIGHT: 右\\n> 当position_x和position_y参数值存在时,position不生效\",\n      \"default\": \"MIDDLE\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"LEFT\",\n        \"MIDDLE\",\n        \"RIGHT\"\n      ],\n      \"maxLength\": 64,\n      \"minLength\": 1,\n      \"example\": \"MIDDLE\"\n    },\n    \"position_x\": {\n      \"description\": \"分身数字人X轴位置,即分身数字图片底边中心点像素的X轴的像素值。\\n> 横屏(16:9)背景图片像素为1920x1080;竖屏(9:16)背景图片像素为1080x1920。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 3840.0,\n      \"minimum\": -1920.0\n    },\n    \"position_y\": {\n      \"description\": \"分身数字Y轴位置,即分身数字图片底边中心点像素的Y轴的像素值。\\n> 横屏(16:9)背景图片像素为1920x1080;竖屏(9:16)背景图片像素为1080x1920。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 3840.0,\n      \"minimum\": -1920.0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HumanPosition2D {
    #[doc = "分身数字人在背景图片中的位置。\n* LEFT: 左\n* MIDDLE: 中\n* RIGHT: 右\n> 当position_x和position_y参数值存在时,position不生效"]
    #[serde(default = "defaults::human_position2_d_position")]
    pub position: HumanPosition2DPosition,
    #[doc = "分身数字人X轴位置,即分身数字图片底边中心点像素的X轴的像素值。\n> 横屏(16:9)背景图片像素为1920x1080;竖屏(9:16)背景图片像素为1080x1920。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_x: Option<i64>,
    #[doc = "分身数字Y轴位置,即分身数字图片底边中心点像素的Y轴的像素值。\n> 横屏(16:9)背景图片像素为1920x1080;竖屏(9:16)背景图片像素为1080x1920。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_y: Option<i64>,
}
impl From<&HumanPosition2D> for HumanPosition2D {
    fn from(value: &HumanPosition2D) -> Self {
        value.clone()
    }
}
impl HumanPosition2D {
    pub fn builder() -> builder::HumanPosition2D {
        Default::default()
    }
}
#[doc = "分身数字人在背景图片中的位置。\n* LEFT: 左\n* MIDDLE: 中\n* RIGHT: 右\n> 当position_x和position_y参数值存在时,position不生效"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"分身数字人在背景图片中的位置。\\n* LEFT: 左\\n* MIDDLE: 中\\n* RIGHT: 右\\n> 当position_x和position_y参数值存在时,position不生效\",\n  \"default\": \"MIDDLE\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"LEFT\",\n    \"MIDDLE\",\n    \"RIGHT\"\n  ],\n  \"maxLength\": 64,\n  \"minLength\": 1,\n  \"example\": \"MIDDLE\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum HumanPosition2DPosition {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "MIDDLE")]
    Middle,
    #[serde(rename = "RIGHT")]
    Right,
}
impl From<&HumanPosition2DPosition> for HumanPosition2DPosition {
    fn from(value: &HumanPosition2DPosition) -> Self {
        value.clone()
    }
}
impl ToString for HumanPosition2DPosition {
    fn to_string(&self) -> String {
        match *self {
            Self::Left => "LEFT".to_string(),
            Self::Middle => "MIDDLE".to_string(),
            Self::Right => "RIGHT".to_string(),
        }
    }
}
impl std::str::FromStr for HumanPosition2DPosition {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "LEFT" => Ok(Self::Left),
            "MIDDLE" => Ok(Self::Middle),
            "RIGHT" => Ok(Self::Right),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for HumanPosition2DPosition {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for HumanPosition2DPosition {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for HumanPosition2DPosition {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for HumanPosition2DPosition {
    fn default() -> Self {
        HumanPosition2DPosition::Middle
    }
}
#[doc = "分身数字人在背景图片中的大小。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"分身数字人在背景图片中的大小。\",\n  \"properties\": {\n    \"height\": {\n      \"description\": \"分身数字人高度像素值。\\n> 横屏(16:9)背景图片像素为1920x1080;竖屏(9:16)背景图片像素为1080x1920。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 7680.0,\n      \"minimum\": 1.0\n    },\n    \"width\": {\n      \"description\": \"分身数字人宽度像素值。\\n> 横屏(16:9)背景图片像素为1920x1080;竖屏(9:16)背景图片像素为1080x1920。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 7680.0,\n      \"minimum\": 1.0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HumanSize2D {
    #[doc = "分身数字人高度像素值。\n> 横屏(16:9)背景图片像素为1920x1080;竖屏(9:16)背景图片像素为1080x1920。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[doc = "分身数字人宽度像素值。\n> 横屏(16:9)背景图片像素为1920x1080;竖屏(9:16)背景图片像素为1080x1920。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}
impl From<&HumanSize2D> for HumanSize2D {
    fn from(value: &HumanSize2D) -> Self {
        value.clone()
    }
}
impl HumanSize2D {
    pub fn builder() -> builder::HumanSize2D {
        Default::default()
    }
}
#[doc = "素材图片图层配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"素材图片图层配置。\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"image_url\": {\n      \"description\": \"图片文件的URL。\",\n      \"type\": \"string\",\n      \"maxLength\": 2048,\n      \"minLength\": 1\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageLayerConfig {
    #[doc = "图片文件的URL。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ImageLayerConfigImageUrl>,
}
impl From<&ImageLayerConfig> for ImageLayerConfig {
    fn from(value: &ImageLayerConfig) -> Self {
        value.clone()
    }
}
impl ImageLayerConfig {
    pub fn builder() -> builder::ImageLayerConfig {
        Default::default()
    }
}
#[doc = "图片文件的URL。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"图片文件的URL。\",\n  \"type\": \"string\",\n  \"maxLength\": 2048,\n  \"minLength\": 1\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ImageLayerConfigImageUrl(String);
impl std::ops::Deref for ImageLayerConfigImageUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ImageLayerConfigImageUrl> for String {
    fn from(value: ImageLayerConfigImageUrl) -> Self {
        value.0
    }
}
impl From<&ImageLayerConfigImageUrl> for ImageLayerConfigImageUrl {
    fn from(value: &ImageLayerConfigImageUrl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ImageLayerConfigImageUrl {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 2048usize {
            return Err("longer than 2048 characters");
        }
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for ImageLayerConfigImageUrl {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ImageLayerConfigImageUrl {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ImageLayerConfigImageUrl {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ImageLayerConfigImageUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "互动规则配置信息"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"互动规则配置信息\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"enabled\": {\n      \"description\": \"是否启用\",\n      \"type\": \"boolean\"\n    },\n    \"event_type\": {\n      \"description\": \"事件类型。 * 1:弹幕事件 * 2:用户入场事件 * 3:用户点赞事件 * 4:用户送礼事件 * 10: 预置话术事件\",\n      \"type\": \"integer\",\n      \"maximum\": 100.0,\n      \"minimum\": 0.0\n    },\n    \"hit_condition\": {\n      \"description\": \"命中条件\",\n      \"$ref\": \"#/definitions/HitCondition\"\n    },\n    \"review_config\": {\n      \"description\": \"内容审核配置\",\n      \"$ref\": \"#/definitions/ReviewConfig\"\n    },\n    \"rule_index\": {\n      \"description\": \"规则索引\",\n      \"type\": \"string\",\n      \"maxLength\": 64,\n      \"minLength\": 0\n    },\n    \"rule_name\": {\n      \"description\": \"规则名称\",\n      \"type\": \"string\",\n      \"maxLength\": 256,\n      \"minLength\": 0\n    },\n    \"trigger\": {\n      \"description\": \"触发器\",\n      \"$ref\": \"#/definitions/TriggerProcess\"\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InteractionRuleInfo {
    #[doc = "是否启用"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "事件类型。 * 1:弹幕事件 * 2:用户入场事件 * 3:用户点赞事件 * 4:用户送礼事件 * 10: 预置话术事件"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<i64>,
    #[doc = "命中条件"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit_condition: Option<HitCondition>,
    #[doc = "内容审核配置"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_config: Option<ReviewConfig>,
    #[doc = "规则索引"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_index: Option<InteractionRuleInfoRuleIndex>,
    #[doc = "规则名称"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<InteractionRuleInfoRuleName>,
    #[doc = "触发器"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<TriggerProcess>,
}
impl From<&InteractionRuleInfo> for InteractionRuleInfo {
    fn from(value: &InteractionRuleInfo) -> Self {
        value.clone()
    }
}
impl InteractionRuleInfo {
    pub fn builder() -> builder::InteractionRuleInfo {
        Default::default()
    }
}
#[doc = "规则索引"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"规则索引\",\n  \"type\": \"string\",\n  \"maxLength\": 64,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InteractionRuleInfoRuleIndex(String);
impl std::ops::Deref for InteractionRuleInfoRuleIndex {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<InteractionRuleInfoRuleIndex> for String {
    fn from(value: InteractionRuleInfoRuleIndex) -> Self {
        value.0
    }
}
impl From<&InteractionRuleInfoRuleIndex> for InteractionRuleInfoRuleIndex {
    fn from(value: &InteractionRuleInfoRuleIndex) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for InteractionRuleInfoRuleIndex {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 64usize {
            return Err("longer than 64 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for InteractionRuleInfoRuleIndex {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InteractionRuleInfoRuleIndex {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InteractionRuleInfoRuleIndex {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for InteractionRuleInfoRuleIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "规则名称"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"规则名称\",\n  \"type\": \"string\",\n  \"maxLength\": 256,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InteractionRuleInfoRuleName(String);
impl std::ops::Deref for InteractionRuleInfoRuleName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<InteractionRuleInfoRuleName> for String {
    fn from(value: InteractionRuleInfoRuleName) -> Self {
        value.0
    }
}
impl From<&InteractionRuleInfoRuleName> for InteractionRuleInfoRuleName {
    fn from(value: &InteractionRuleInfoRuleName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for InteractionRuleInfoRuleName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 256usize {
            return Err("longer than 256 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for InteractionRuleInfoRuleName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InteractionRuleInfoRuleName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InteractionRuleInfoRuleName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for InteractionRuleInfoRuleName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "图层配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"图层配置。\",\n  \"required\": [\n    \"layer_type\",\n    \"position\"\n  ],\n  \"properties\": {\n    \"group_id\": {\n      \"description\": \"多场景素材编组。同一group_id的素材,在应用全局时共享位置信息。\",\n      \"type\": \"string\",\n      \"maxLength\": 64,\n      \"minLength\": 0\n    },\n    \"image_config\": {\n      \"$ref\": \"#/definitions/ImageLayerConfig\"\n    },\n    \"layer_type\": {\n      \"description\": \"图层类型。\\n- HUMAN:  人物图层\\n- IMAGE: 素材图片图层\\n- VIDEO: 素材视频图层\\n- TEXT: 素材文字图层\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"HUMAN\",\n        \"IMAGE\",\n        \"VIDEO\",\n        \"TEXT\"\n      ]\n    },\n    \"position\": {\n      \"$ref\": \"#/definitions/LayerPositionConfig\"\n    },\n    \"size\": {\n      \"$ref\": \"#/definitions/LayerSizeConfig\"\n    },\n    \"text_config\": {\n      \"$ref\": \"#/definitions/TextLayerConfig\"\n    },\n    \"video_config\": {\n      \"$ref\": \"#/definitions/VideoLayerConfig\"\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LayerConfig {
    #[doc = "多场景素材编组。同一group_id的素材,在应用全局时共享位置信息。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<LayerConfigGroupId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_config: Option<ImageLayerConfig>,
    #[doc = "图层类型。\n- HUMAN:  人物图层\n- IMAGE: 素材图片图层\n- VIDEO: 素材视频图层\n- TEXT: 素材文字图层"]
    pub layer_type: LayerConfigLayerType,
    pub position: LayerPositionConfig,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<LayerSizeConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_config: Option<TextLayerConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_config: Option<VideoLayerConfig>,
}
impl From<&LayerConfig> for LayerConfig {
    fn from(value: &LayerConfig) -> Self {
        value.clone()
    }
}
impl LayerConfig {
    pub fn builder() -> builder::LayerConfig {
        Default::default()
    }
}
#[doc = "多场景素材编组。同一group_id的素材,在应用全局时共享位置信息。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"多场景素材编组。同一group_id的素材,在应用全局时共享位置信息。\",\n  \"type\": \"string\",\n  \"maxLength\": 64,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LayerConfigGroupId(String);
impl std::ops::Deref for LayerConfigGroupId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LayerConfigGroupId> for String {
    fn from(value: LayerConfigGroupId) -> Self {
        value.0
    }
}
impl From<&LayerConfigGroupId> for LayerConfigGroupId {
    fn from(value: &LayerConfigGroupId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LayerConfigGroupId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 64usize {
            return Err("longer than 64 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LayerConfigGroupId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LayerConfigGroupId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LayerConfigGroupId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LayerConfigGroupId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "图层类型。\n- HUMAN:  人物图层\n- IMAGE: 素材图片图层\n- VIDEO: 素材视频图层\n- TEXT: 素材文字图层"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"图层类型。\\n- HUMAN:  人物图层\\n- IMAGE: 素材图片图层\\n- VIDEO: 素材视频图层\\n- TEXT: 素材文字图层\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"HUMAN\",\n    \"IMAGE\",\n    \"VIDEO\",\n    \"TEXT\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LayerConfigLayerType {
    #[serde(rename = "HUMAN")]
    Human,
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "VIDEO")]
    Video,
    #[serde(rename = "TEXT")]
    Text,
}
impl From<&LayerConfigLayerType> for LayerConfigLayerType {
    fn from(value: &LayerConfigLayerType) -> Self {
        value.clone()
    }
}
impl ToString for LayerConfigLayerType {
    fn to_string(&self) -> String {
        match *self {
            Self::Human => "HUMAN".to_string(),
            Self::Image => "IMAGE".to_string(),
            Self::Video => "VIDEO".to_string(),
            Self::Text => "TEXT".to_string(),
        }
    }
}
impl std::str::FromStr for LayerConfigLayerType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "HUMAN" => Ok(Self::Human),
            "IMAGE" => Ok(Self::Image),
            "VIDEO" => Ok(Self::Video),
            "TEXT" => Ok(Self::Text),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LayerConfigLayerType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LayerConfigLayerType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LayerConfigLayerType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "图层位置配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"图层位置配置。\",\n  \"type\": \"object\",\n  \"required\": [\n    \"dx\",\n    \"dy\",\n    \"layer_index\"\n  ],\n  \"properties\": {\n    \"dx\": {\n      \"description\": \"图层左上角像素点的X轴位置值(画布左上角坐标是0x0)。\\n> * 横屏(16:9)画布像素为1920x1080;竖屏(9:16)画布像素为1080x1920。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 3840.0,\n      \"minimum\": -1920.0,\n      \"example\": 256\n    },\n    \"dy\": {\n      \"description\": \"图层图片左上角像素点的Y轴位置值(画布左上角坐标是0x0)。\\n> 横屏(16:9)画布像素为1920x1080;竖屏(9:16)画布像素为1080x1920。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 3840.0,\n      \"minimum\": -1920.0,\n      \"example\": 512\n    },\n    \"layer_index\": {\n      \"description\": \"图片/视频/人物图的层顺序。\\n> * 图层顺序从1开始的整数,底层图层顺序是1,往上依次增加。\",\n      \"type\": \"integer\",\n      \"maximum\": 100.0,\n      \"minimum\": 1.0,\n      \"example\": 1\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LayerPositionConfig {
    #[doc = "图层左上角像素点的X轴位置值(画布左上角坐标是0x0)。\n> * 横屏(16:9)画布像素为1920x1080;竖屏(9:16)画布像素为1080x1920。"]
    pub dx: i64,
    #[doc = "图层图片左上角像素点的Y轴位置值(画布左上角坐标是0x0)。\n> 横屏(16:9)画布像素为1920x1080;竖屏(9:16)画布像素为1080x1920。"]
    pub dy: i64,
    #[doc = "图片/视频/人物图的层顺序。\n> * 图层顺序从1开始的整数,底层图层顺序是1,往上依次增加。"]
    pub layer_index: i64,
}
impl From<&LayerPositionConfig> for LayerPositionConfig {
    fn from(value: &LayerPositionConfig) -> Self {
        value.clone()
    }
}
impl LayerPositionConfig {
    pub fn builder() -> builder::LayerPositionConfig {
        Default::default()
    }
}
#[doc = "图层大小配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"图层大小配置。\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"height\": {\n      \"description\": \"图层图片高度像素值(相对画布大小)。\\n> 横屏(16:9)画布像素为1920x1080;竖屏(9:16)画布像素为1080x1920。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 7680.0,\n      \"minimum\": 1.0\n    },\n    \"width\": {\n      \"description\": \"图层图片宽度像素值(相对画布大小)。\\n> 横屏(16:9)画布像素为1920x1080;竖屏(9:16)画布像素为1080x1920。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 7680.0,\n      \"minimum\": 1.0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LayerSizeConfig {
    #[doc = "图层图片高度像素值(相对画布大小)。\n> 横屏(16:9)画布像素为1920x1080;竖屏(9:16)画布像素为1080x1920。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[doc = "图层图片宽度像素值(相对画布大小)。\n> 横屏(16:9)画布像素为1920x1080;竖屏(9:16)画布像素为1080x1920。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}
impl From<&LayerSizeConfig> for LayerSizeConfig {
    fn from(value: &LayerSizeConfig) -> Self {
        value.clone()
    }
}
impl LayerSizeConfig {
    pub fn builder() -> builder::LayerSizeConfig {
        Default::default()
    }
}
#[doc = "直播音频配置"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"直播音频配置\",\n  \"properties\": {\n    \"audio_url\": {\n      \"description\": \"音频URL。仅支持MP3格式,大小<100MB。输出会自动转化为单声道16KHZ采样。\",\n      \"type\": \"string\",\n      \"maxLength\": 2048,\n      \"minLength\": 0\n    },\n    \"subtitle_url\": {\n      \"description\": \"音频对应的字幕文件URL。仅SRT格式,大小<1MB。\",\n      \"type\": \"string\",\n      \"maxLength\": 2048,\n      \"minLength\": 0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveAudioConfig {
    #[doc = "音频URL。仅支持MP3格式,大小<100MB。输出会自动转化为单声道16KHZ采样。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_url: Option<LiveAudioConfigAudioUrl>,
    #[doc = "音频对应的字幕文件URL。仅SRT格式,大小<1MB。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle_url: Option<LiveAudioConfigSubtitleUrl>,
}
impl From<&LiveAudioConfig> for LiveAudioConfig {
    fn from(value: &LiveAudioConfig) -> Self {
        value.clone()
    }
}
impl LiveAudioConfig {
    pub fn builder() -> builder::LiveAudioConfig {
        Default::default()
    }
}
#[doc = "音频URL。仅支持MP3格式,大小<100MB。输出会自动转化为单声道16KHZ采样。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"音频URL。仅支持MP3格式,大小<100MB。输出会自动转化为单声道16KHZ采样。\",\n  \"type\": \"string\",\n  \"maxLength\": 2048,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LiveAudioConfigAudioUrl(String);
impl std::ops::Deref for LiveAudioConfigAudioUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LiveAudioConfigAudioUrl> for String {
    fn from(value: LiveAudioConfigAudioUrl) -> Self {
        value.0
    }
}
impl From<&LiveAudioConfigAudioUrl> for LiveAudioConfigAudioUrl {
    fn from(value: &LiveAudioConfigAudioUrl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LiveAudioConfigAudioUrl {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 2048usize {
            return Err("longer than 2048 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LiveAudioConfigAudioUrl {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiveAudioConfigAudioUrl {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiveAudioConfigAudioUrl {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LiveAudioConfigAudioUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "音频对应的字幕文件URL。仅SRT格式,大小<1MB。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"音频对应的字幕文件URL。仅SRT格式,大小<1MB。\",\n  \"type\": \"string\",\n  \"maxLength\": 2048,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LiveAudioConfigSubtitleUrl(String);
impl std::ops::Deref for LiveAudioConfigSubtitleUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LiveAudioConfigSubtitleUrl> for String {
    fn from(value: LiveAudioConfigSubtitleUrl) -> Self {
        value.0
    }
}
impl From<&LiveAudioConfigSubtitleUrl> for LiveAudioConfigSubtitleUrl {
    fn from(value: &LiveAudioConfigSubtitleUrl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LiveAudioConfigSubtitleUrl {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 2048usize {
            return Err("longer than 2048 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LiveAudioConfigSubtitleUrl {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiveAudioConfigSubtitleUrl {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiveAudioConfigSubtitleUrl {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LiveAudioConfigSubtitleUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "直播事件回调通知配置"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"直播事件回调通知配置\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"auth_type\": {\n      \"description\": \"认证类型。\\n* NONE。URL中自带认证。\\n* MSS_A。HMACSHA256签名模式,在URL中追加参数:hwSecret,hwTime。取值方式:hwSecret=hmac_sha256(Key, URI(live_event_callback_url)+ hwTime)&hwTime=hex(timestamp)\",\n      \"default\": \"NONE\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"NONE\",\n        \"MSS_A\"\n      ],\n      \"maxLength\": 32,\n      \"minLength\": 0\n    },\n    \"callback_event_type\": {\n      \"description\": \"回调的直播事件类型列表\",\n      \"type\": \"array\",\n      \"items\": {\n        \"description\": \"回调的直播事件类型\",\n        \"type\": \"string\",\n        \"maxLength\": 32,\n        \"minLength\": 1\n      },\n      \"maxItems\": 10,\n      \"minItems\": 1\n    },\n    \"key\": {\n      \"description\": \"密钥Key\",\n      \"type\": \"string\",\n      \"maxLength\": 32,\n      \"minLength\": 0\n    },\n    \"live_event_type_callback_url\": {\n      \"description\": \"直播事件回调地址。https地址,需自带鉴权串。\",\n      \"type\": \"string\",\n      \"maxLength\": 2048,\n      \"minLength\": 0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveEventCallBackConfig {
    #[doc = "认证类型。\n* NONE。URL中自带认证。\n* MSS_A。HMACSHA256签名模式,在URL中追加参数:hwSecret,hwTime。取值方式:hwSecret=hmac_sha256(Key, URI(live_event_callback_url)+ hwTime)&hwTime=hex(timestamp)"]
    #[serde(default = "defaults::live_event_call_back_config_auth_type")]
    pub auth_type: LiveEventCallBackConfigAuthType,
    #[doc = "回调的直播事件类型列表"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub callback_event_type: Vec<LiveEventCallBackConfigCallbackEventTypeItem>,
    #[doc = "密钥Key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<LiveEventCallBackConfigKey>,
    #[doc = "直播事件回调地址。https地址,需自带鉴权串。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_event_type_callback_url: Option<LiveEventCallBackConfigLiveEventTypeCallbackUrl>,
}
impl From<&LiveEventCallBackConfig> for LiveEventCallBackConfig {
    fn from(value: &LiveEventCallBackConfig) -> Self {
        value.clone()
    }
}
impl LiveEventCallBackConfig {
    pub fn builder() -> builder::LiveEventCallBackConfig {
        Default::default()
    }
}
#[doc = "认证类型。\n* NONE。URL中自带认证。\n* MSS_A。HMACSHA256签名模式,在URL中追加参数:hwSecret,hwTime。取值方式:hwSecret=hmac_sha256(Key, URI(live_event_callback_url)+ hwTime)&hwTime=hex(timestamp)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"认证类型。\\n* NONE。URL中自带认证。\\n* MSS_A。HMACSHA256签名模式,在URL中追加参数:hwSecret,hwTime。取值方式:hwSecret=hmac_sha256(Key, URI(live_event_callback_url)+ hwTime)&hwTime=hex(timestamp)\",\n  \"default\": \"NONE\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"NONE\",\n    \"MSS_A\"\n  ],\n  \"maxLength\": 32,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LiveEventCallBackConfigAuthType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "MSS_A")]
    MssA,
}
impl From<&LiveEventCallBackConfigAuthType> for LiveEventCallBackConfigAuthType {
    fn from(value: &LiveEventCallBackConfigAuthType) -> Self {
        value.clone()
    }
}
impl ToString for LiveEventCallBackConfigAuthType {
    fn to_string(&self) -> String {
        match *self {
            Self::None => "NONE".to_string(),
            Self::MssA => "MSS_A".to_string(),
        }
    }
}
impl std::str::FromStr for LiveEventCallBackConfigAuthType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "NONE" => Ok(Self::None),
            "MSS_A" => Ok(Self::MssA),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LiveEventCallBackConfigAuthType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiveEventCallBackConfigAuthType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiveEventCallBackConfigAuthType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for LiveEventCallBackConfigAuthType {
    fn default() -> Self {
        LiveEventCallBackConfigAuthType::None
    }
}
#[doc = "回调的直播事件类型"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"回调的直播事件类型\",\n  \"type\": \"string\",\n  \"maxLength\": 32,\n  \"minLength\": 1\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LiveEventCallBackConfigCallbackEventTypeItem(String);
impl std::ops::Deref for LiveEventCallBackConfigCallbackEventTypeItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LiveEventCallBackConfigCallbackEventTypeItem> for String {
    fn from(value: LiveEventCallBackConfigCallbackEventTypeItem) -> Self {
        value.0
    }
}
impl From<&LiveEventCallBackConfigCallbackEventTypeItem>
    for LiveEventCallBackConfigCallbackEventTypeItem
{
    fn from(value: &LiveEventCallBackConfigCallbackEventTypeItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LiveEventCallBackConfigCallbackEventTypeItem {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 32usize {
            return Err("longer than 32 characters");
        }
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LiveEventCallBackConfigCallbackEventTypeItem {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiveEventCallBackConfigCallbackEventTypeItem {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiveEventCallBackConfigCallbackEventTypeItem {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LiveEventCallBackConfigCallbackEventTypeItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "密钥Key"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"密钥Key\",\n  \"type\": \"string\",\n  \"maxLength\": 32,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LiveEventCallBackConfigKey(String);
impl std::ops::Deref for LiveEventCallBackConfigKey {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LiveEventCallBackConfigKey> for String {
    fn from(value: LiveEventCallBackConfigKey) -> Self {
        value.0
    }
}
impl From<&LiveEventCallBackConfigKey> for LiveEventCallBackConfigKey {
    fn from(value: &LiveEventCallBackConfigKey) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LiveEventCallBackConfigKey {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 32usize {
            return Err("longer than 32 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LiveEventCallBackConfigKey {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiveEventCallBackConfigKey {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiveEventCallBackConfigKey {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LiveEventCallBackConfigKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "直播事件回调地址。https地址,需自带鉴权串。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"直播事件回调地址。https地址,需自带鉴权串。\",\n  \"type\": \"string\",\n  \"maxLength\": 2048,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LiveEventCallBackConfigLiveEventTypeCallbackUrl(String);
impl std::ops::Deref for LiveEventCallBackConfigLiveEventTypeCallbackUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LiveEventCallBackConfigLiveEventTypeCallbackUrl> for String {
    fn from(value: LiveEventCallBackConfigLiveEventTypeCallbackUrl) -> Self {
        value.0
    }
}
impl From<&LiveEventCallBackConfigLiveEventTypeCallbackUrl>
    for LiveEventCallBackConfigLiveEventTypeCallbackUrl
{
    fn from(value: &LiveEventCallBackConfigLiveEventTypeCallbackUrl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LiveEventCallBackConfigLiveEventTypeCallbackUrl {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 2048usize {
            return Err("longer than 2048 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LiveEventCallBackConfigLiveEventTypeCallbackUrl {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiveEventCallBackConfigLiveEventTypeCallbackUrl {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiveEventCallBackConfigLiveEventTypeCallbackUrl {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LiveEventCallBackConfigLiveEventTypeCallbackUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "直播话术配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"直播话术配置。\",\n  \"properties\": {\n    \"audio_config\": {\n      \"$ref\": \"#/definitions/LiveAudioConfig\"\n    },\n    \"sequence_no\": {\n      \"description\": \"剧本序号。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 2147483647.0,\n      \"minimum\": 0.0,\n      \"example\": 1\n    },\n    \"text_config\": {\n      \"description\": \"话术配置。\",\n      \"$ref\": \"#/definitions/TextConfig\"\n    },\n    \"title\": {\n      \"description\": \"段落标题。\",\n      \"type\": \"string\",\n      \"maxLength\": 256,\n      \"minLength\": 0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveShootScriptItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_config: Option<LiveAudioConfig>,
    #[doc = "剧本序号。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence_no: Option<i64>,
    #[doc = "话术配置。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_config: Option<TextConfig>,
    #[doc = "段落标题。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<LiveShootScriptItemTitle>,
}
impl From<&LiveShootScriptItem> for LiveShootScriptItem {
    fn from(value: &LiveShootScriptItem) -> Self {
        value.clone()
    }
}
impl LiveShootScriptItem {
    pub fn builder() -> builder::LiveShootScriptItem {
        Default::default()
    }
}
#[doc = "段落标题。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"段落标题。\",\n  \"type\": \"string\",\n  \"maxLength\": 256,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LiveShootScriptItemTitle(String);
impl std::ops::Deref for LiveShootScriptItemTitle {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LiveShootScriptItemTitle> for String {
    fn from(value: LiveShootScriptItemTitle) -> Self {
        value.0
    }
}
impl From<&LiveShootScriptItemTitle> for LiveShootScriptItemTitle {
    fn from(value: &LiveShootScriptItemTitle) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LiveShootScriptItemTitle {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 256usize {
            return Err("longer than 256 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LiveShootScriptItemTitle {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiveShootScriptItemTitle {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiveShootScriptItemTitle {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LiveShootScriptItemTitle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "创视频制作脚本请求。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"创视频制作脚本请求。\",\n  \"type\": \"object\",\n  \"required\": [\n    \"script_name\",\n    \"shoot_scripts\"\n  ],\n  \"properties\": {\n    \"background_config\": {\n      \"description\": \"背景配置。\",\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/definitions/BackgroundConfigInfo\"\n      },\n      \"maxItems\": 10,\n      \"minItems\": 0\n    },\n    \"dh_id\": {\n      \"description\": \"数字人ID。对应形象和音色组合。\",\n      \"type\": \"string\",\n      \"maxLength\": 64,\n      \"minLength\": 0,\n      \"example\": \"b5ec8585fb42699cc9b1bac87787d3a7c\"\n    },\n    \"layer_config\": {\n      \"description\": \"图层配置。\",\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/definitions/LayerConfig\"\n      },\n      \"maxItems\": 100,\n      \"minItems\": 0\n    },\n    \"model_asset_id\": {\n      \"description\": \"数字人模型资产ID。\",\n      \"type\": \"string\",\n      \"maxLength\": 64,\n      \"minLength\": 0,\n      \"example\": \"a5d295cdb345c11bd9f36bc22ced3a7a\"\n    },\n    \"script_description\": {\n      \"description\": \"剧本描述。\",\n      \"type\": \"string\",\n      \"maxLength\": 1024,\n      \"minLength\": 0,\n      \"example\": \"课件\"\n    },\n    \"script_name\": {\n      \"description\": \"剧本名称\",\n      \"type\": \"string\",\n      \"maxLength\": 256,\n      \"minLength\": 1,\n      \"example\": \"大自然的传说\"\n    },\n    \"shoot_scripts\": {\n      \"description\": \"拍摄脚本列表。\",\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/definitions/LiveShootScriptItem\"\n      },\n      \"maxItems\": 100,\n      \"minItems\": 0\n    },\n    \"voice_config\": {\n      \"description\": \"语音配置参数。\",\n      \"$ref\": \"#/definitions/VoiceConfig\"\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveVideoScriptInfo {
    #[doc = "背景配置。"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub background_config: Vec<BackgroundConfigInfo>,
    #[doc = "数字人ID。对应形象和音色组合。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dh_id: Option<LiveVideoScriptInfoDhId>,
    #[doc = "图层配置。"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layer_config: Vec<LayerConfig>,
    #[doc = "数字人模型资产ID。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_asset_id: Option<LiveVideoScriptInfoModelAssetId>,
    #[doc = "剧本描述。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_description: Option<LiveVideoScriptInfoScriptDescription>,
    #[doc = "剧本名称"]
    pub script_name: LiveVideoScriptInfoScriptName,
    #[doc = "拍摄脚本列表。"]
    pub shoot_scripts: Vec<LiveShootScriptItem>,
    #[doc = "语音配置参数。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice_config: Option<VoiceConfig>,
}
impl From<&LiveVideoScriptInfo> for LiveVideoScriptInfo {
    fn from(value: &LiveVideoScriptInfo) -> Self {
        value.clone()
    }
}
impl LiveVideoScriptInfo {
    pub fn builder() -> builder::LiveVideoScriptInfo {
        Default::default()
    }
}
#[doc = "数字人ID。对应形象和音色组合。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"数字人ID。对应形象和音色组合。\",\n  \"type\": \"string\",\n  \"maxLength\": 64,\n  \"minLength\": 0,\n  \"example\": \"b5ec8585fb42699cc9b1bac87787d3a7c\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LiveVideoScriptInfoDhId(String);
impl std::ops::Deref for LiveVideoScriptInfoDhId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LiveVideoScriptInfoDhId> for String {
    fn from(value: LiveVideoScriptInfoDhId) -> Self {
        value.0
    }
}
impl From<&LiveVideoScriptInfoDhId> for LiveVideoScriptInfoDhId {
    fn from(value: &LiveVideoScriptInfoDhId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LiveVideoScriptInfoDhId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 64usize {
            return Err("longer than 64 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LiveVideoScriptInfoDhId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiveVideoScriptInfoDhId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiveVideoScriptInfoDhId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LiveVideoScriptInfoDhId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "数字人模型资产ID。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"数字人模型资产ID。\",\n  \"type\": \"string\",\n  \"maxLength\": 64,\n  \"minLength\": 0,\n  \"example\": \"a5d295cdb345c11bd9f36bc22ced3a7a\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LiveVideoScriptInfoModelAssetId(String);
impl std::ops::Deref for LiveVideoScriptInfoModelAssetId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LiveVideoScriptInfoModelAssetId> for String {
    fn from(value: LiveVideoScriptInfoModelAssetId) -> Self {
        value.0
    }
}
impl From<&LiveVideoScriptInfoModelAssetId> for LiveVideoScriptInfoModelAssetId {
    fn from(value: &LiveVideoScriptInfoModelAssetId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LiveVideoScriptInfoModelAssetId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 64usize {
            return Err("longer than 64 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LiveVideoScriptInfoModelAssetId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiveVideoScriptInfoModelAssetId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiveVideoScriptInfoModelAssetId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LiveVideoScriptInfoModelAssetId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "剧本描述。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"剧本描述。\",\n  \"type\": \"string\",\n  \"maxLength\": 1024,\n  \"minLength\": 0,\n  \"example\": \"课件\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LiveVideoScriptInfoScriptDescription(String);
impl std::ops::Deref for LiveVideoScriptInfoScriptDescription {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LiveVideoScriptInfoScriptDescription> for String {
    fn from(value: LiveVideoScriptInfoScriptDescription) -> Self {
        value.0
    }
}
impl From<&LiveVideoScriptInfoScriptDescription> for LiveVideoScriptInfoScriptDescription {
    fn from(value: &LiveVideoScriptInfoScriptDescription) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LiveVideoScriptInfoScriptDescription {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 1024usize {
            return Err("longer than 1024 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LiveVideoScriptInfoScriptDescription {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiveVideoScriptInfoScriptDescription {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiveVideoScriptInfoScriptDescription {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LiveVideoScriptInfoScriptDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "剧本名称"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"剧本名称\",\n  \"type\": \"string\",\n  \"maxLength\": 256,\n  \"minLength\": 1,\n  \"example\": \"大自然的传说\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LiveVideoScriptInfoScriptName(String);
impl std::ops::Deref for LiveVideoScriptInfoScriptName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LiveVideoScriptInfoScriptName> for String {
    fn from(value: LiveVideoScriptInfoScriptName) -> Self {
        value.0
    }
}
impl From<&LiveVideoScriptInfoScriptName> for LiveVideoScriptInfoScriptName {
    fn from(value: &LiveVideoScriptInfoScriptName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LiveVideoScriptInfoScriptName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 256usize {
            return Err("longer than 256 characters");
        }
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LiveVideoScriptInfoScriptName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiveVideoScriptInfoScriptName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiveVideoScriptInfoScriptName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LiveVideoScriptInfoScriptName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "剧本播放策略"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"剧本播放策略\",\n  \"properties\": {\n    \"auto_play_script\": {\n      \"description\": \"是否自动播放剧本。\\ntrue: 服务完成任务初始化后,自动播放剧本\\nfalse: 服务完成任务初始化后,等待信号后再开始播放剧本\",\n      \"default\": true,\n      \"type\": \"boolean\"\n    },\n    \"play_mode\": {\n      \"description\": \"驱动方式。默认TEXT\\n* TEXT: 文本驱动,即通过TTS合成语音\\n* AUDIO: 语音驱动\",\n      \"default\": \"TEXT\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"TEXT\",\n        \"AUDIO\"\n      ],\n      \"maxLength\": 32,\n      \"minLength\": 0\n    },\n    \"random_play_mode\": {\n      \"description\": \"随机播报模式。\\n* NONE: 不启动随机播报。\\n* SCENE: 按场景随机播报。场景内段落按顺序播报。\\n* SCRIPT_ITEM:按段落随机播报。场景按顺序播报。\\n* SCENE_AND_SCRIPT_ITEM: 场景和段落都随机播报。\",\n      \"default\": \"SCRIPT_ITEM\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"NONE\",\n        \"SCENE\",\n        \"SCRIPT_ITEM\",\n        \"SCENE_AND_SCRIPT_ITEM\"\n      ],\n      \"maxLength\": 32,\n      \"minLength\": 0\n    },\n    \"repeat_count\": {\n      \"description\": \"剧本重复播放次数。\\n-1表示持续重复,直至人工停止\\n0 表示不重复,仅执行一次\\n其他值n,实际运行次数为n+1次\",\n      \"default\": 0,\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 100.0,\n      \"minimum\": -1.0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayPolicy {
    #[doc = "是否自动播放剧本。\ntrue: 服务完成任务初始化后,自动播放剧本\nfalse: 服务完成任务初始化后,等待信号后再开始播放剧本"]
    #[serde(default = "defaults::default_bool::<true>")]
    pub auto_play_script: bool,
    #[doc = "驱动方式。默认TEXT\n* TEXT: 文本驱动,即通过TTS合成语音\n* AUDIO: 语音驱动"]
    #[serde(default = "defaults::play_policy_play_mode")]
    pub play_mode: PlayPolicyPlayMode,
    #[doc = "随机播报模式。\n* NONE: 不启动随机播报。\n* SCENE: 按场景随机播报。场景内段落按顺序播报。\n* SCRIPT_ITEM:按段落随机播报。场景按顺序播报。\n* SCENE_AND_SCRIPT_ITEM: 场景和段落都随机播报。"]
    #[serde(default = "defaults::play_policy_random_play_mode")]
    pub random_play_mode: PlayPolicyRandomPlayMode,
    #[doc = "剧本重复播放次数。\n-1表示持续重复,直至人工停止\n0 表示不重复,仅执行一次\n其他值n,实际运行次数为n+1次"]
    #[serde(default)]
    pub repeat_count: i64,
}
impl From<&PlayPolicy> for PlayPolicy {
    fn from(value: &PlayPolicy) -> Self {
        value.clone()
    }
}
impl PlayPolicy {
    pub fn builder() -> builder::PlayPolicy {
        Default::default()
    }
}
#[doc = "驱动方式。默认TEXT\n* TEXT: 文本驱动,即通过TTS合成语音\n* AUDIO: 语音驱动"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"驱动方式。默认TEXT\\n* TEXT: 文本驱动,即通过TTS合成语音\\n* AUDIO: 语音驱动\",\n  \"default\": \"TEXT\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"TEXT\",\n    \"AUDIO\"\n  ],\n  \"maxLength\": 32,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum PlayPolicyPlayMode {
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "AUDIO")]
    Audio,
}
impl From<&PlayPolicyPlayMode> for PlayPolicyPlayMode {
    fn from(value: &PlayPolicyPlayMode) -> Self {
        value.clone()
    }
}
impl ToString for PlayPolicyPlayMode {
    fn to_string(&self) -> String {
        match *self {
            Self::Text => "TEXT".to_string(),
            Self::Audio => "AUDIO".to_string(),
        }
    }
}
impl std::str::FromStr for PlayPolicyPlayMode {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "TEXT" => Ok(Self::Text),
            "AUDIO" => Ok(Self::Audio),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for PlayPolicyPlayMode {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PlayPolicyPlayMode {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PlayPolicyPlayMode {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for PlayPolicyPlayMode {
    fn default() -> Self {
        PlayPolicyPlayMode::Text
    }
}
#[doc = "随机播报模式。\n* NONE: 不启动随机播报。\n* SCENE: 按场景随机播报。场景内段落按顺序播报。\n* SCRIPT_ITEM:按段落随机播报。场景按顺序播报。\n* SCENE_AND_SCRIPT_ITEM: 场景和段落都随机播报。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"随机播报模式。\\n* NONE: 不启动随机播报。\\n* SCENE: 按场景随机播报。场景内段落按顺序播报。\\n* SCRIPT_ITEM:按段落随机播报。场景按顺序播报。\\n* SCENE_AND_SCRIPT_ITEM: 场景和段落都随机播报。\",\n  \"default\": \"SCRIPT_ITEM\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"NONE\",\n    \"SCENE\",\n    \"SCRIPT_ITEM\",\n    \"SCENE_AND_SCRIPT_ITEM\"\n  ],\n  \"maxLength\": 32,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum PlayPolicyRandomPlayMode {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "SCENE")]
    Scene,
    #[serde(rename = "SCRIPT_ITEM")]
    ScriptItem,
    #[serde(rename = "SCENE_AND_SCRIPT_ITEM")]
    SceneAndScriptItem,
}
impl From<&PlayPolicyRandomPlayMode> for PlayPolicyRandomPlayMode {
    fn from(value: &PlayPolicyRandomPlayMode) -> Self {
        value.clone()
    }
}
impl ToString for PlayPolicyRandomPlayMode {
    fn to_string(&self) -> String {
        match *self {
            Self::None => "NONE".to_string(),
            Self::Scene => "SCENE".to_string(),
            Self::ScriptItem => "SCRIPT_ITEM".to_string(),
            Self::SceneAndScriptItem => "SCENE_AND_SCRIPT_ITEM".to_string(),
        }
    }
}
impl std::str::FromStr for PlayPolicyRandomPlayMode {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "NONE" => Ok(Self::None),
            "SCENE" => Ok(Self::Scene),
            "SCRIPT_ITEM" => Ok(Self::ScriptItem),
            "SCENE_AND_SCRIPT_ITEM" => Ok(Self::SceneAndScriptItem),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for PlayPolicyRandomPlayMode {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PlayPolicyRandomPlayMode {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PlayPolicyRandomPlayMode {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for PlayPolicyRandomPlayMode {
    fn default() -> Self {
        PlayPolicyRandomPlayMode::ScriptItem
    }
}
#[doc = "回复音频信息"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"回复音频信息\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"audio_name\": {\n      \"description\": \"音频名\",\n      \"type\": \"string\",\n      \"maxLength\": 256,\n      \"minLength\": 0\n    },\n    \"audio_url\": {\n      \"description\": \"音频URL\",\n      \"type\": \"string\",\n      \"maxLength\": 2048,\n      \"minLength\": 0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplyAudioInfo {
    #[doc = "音频名"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_name: Option<ReplyAudioInfoAudioName>,
    #[doc = "音频URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_url: Option<ReplyAudioInfoAudioUrl>,
}
impl From<&ReplyAudioInfo> for ReplyAudioInfo {
    fn from(value: &ReplyAudioInfo) -> Self {
        value.clone()
    }
}
impl ReplyAudioInfo {
    pub fn builder() -> builder::ReplyAudioInfo {
        Default::default()
    }
}
#[doc = "音频名"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"音频名\",\n  \"type\": \"string\",\n  \"maxLength\": 256,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ReplyAudioInfoAudioName(String);
impl std::ops::Deref for ReplyAudioInfoAudioName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ReplyAudioInfoAudioName> for String {
    fn from(value: ReplyAudioInfoAudioName) -> Self {
        value.0
    }
}
impl From<&ReplyAudioInfoAudioName> for ReplyAudioInfoAudioName {
    fn from(value: &ReplyAudioInfoAudioName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ReplyAudioInfoAudioName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 256usize {
            return Err("longer than 256 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for ReplyAudioInfoAudioName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ReplyAudioInfoAudioName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ReplyAudioInfoAudioName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ReplyAudioInfoAudioName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "音频URL"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"音频URL\",\n  \"type\": \"string\",\n  \"maxLength\": 2048,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ReplyAudioInfoAudioUrl(String);
impl std::ops::Deref for ReplyAudioInfoAudioUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ReplyAudioInfoAudioUrl> for String {
    fn from(value: ReplyAudioInfoAudioUrl) -> Self {
        value.0
    }
}
impl From<&ReplyAudioInfoAudioUrl> for ReplyAudioInfoAudioUrl {
    fn from(value: &ReplyAudioInfoAudioUrl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ReplyAudioInfoAudioUrl {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 2048usize {
            return Err("longer than 2048 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for ReplyAudioInfoAudioUrl {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ReplyAudioInfoAudioUrl {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ReplyAudioInfoAudioUrl {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ReplyAudioInfoAudioUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "内容审核配置"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"内容审核配置\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"no_need_review\": {\n      \"description\": \"免审核。 目前仅白名单用户可使用此参数,非白名单用户跟随系统策略审核。\",\n      \"type\": \"boolean\"\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReviewConfig {
    #[doc = "免审核。 目前仅白名单用户可使用此参数,非白名单用户跟随系统策略审核。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_need_review: Option<bool>,
}
impl From<&ReviewConfig> for ReviewConfig {
    fn from(value: &ReviewConfig) -> Self {
        value.clone()
    }
}
impl ReviewConfig {
    pub fn builder() -> builder::ReviewConfig {
        Default::default()
    }
}
#[doc = "共享配置"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"共享配置\",\n  \"properties\": {\n    \"allowed_project_ids\": {\n      \"description\": \"允许访问本资产的租户列表\",\n      \"type\": \"array\",\n      \"items\": {\n        \"type\": \"string\",\n        \"maxLength\": 64,\n        \"minLength\": 1\n      },\n      \"maxItems\": 100,\n      \"minItems\": 0\n    },\n    \"expire_time\": {\n      \"description\": \"共享过期时间。空表示永久不过期。\",\n      \"type\": \"string\",\n      \"maxLength\": 20,\n      \"minLength\": 0\n    },\n    \"shared_state\": {\n      \"description\": \"共享状态。\\n* PUBLISHED: 发布。模板可用。\\n- DRAFT: 草稿。编辑态,仅拥有者可访问。\\n- REVIEW:审核态。不可编辑,仅拥有者/审核人员可查看。\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"PUBLISHED\",\n        \"DRAFT\",\n        \"REVIEW\"\n      ],\n      \"maxLength\": 32,\n      \"minLength\": 0\n    },\n    \"shared_type\": {\n      \"description\": \"共享类型。\\n* PRIVATE: 私有,仅本租户可访问。\\n* PUBLIC: 公开,所有租户可访问。当前仅提供系统资产可公开访问。\\n* SHARED:共享,指定租户可访问。拥有者指定租户可访问。\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"PRIVATE\",\n        \"PUBLIC\",\n        \"SHARED\"\n      ],\n      \"maxLength\": 32,\n      \"minLength\": 0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SharedConfig {
    #[doc = "允许访问本资产的租户列表"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_project_ids: Vec<SharedConfigAllowedProjectIdsItem>,
    #[doc = "共享过期时间。空表示永久不过期。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<SharedConfigExpireTime>,
    #[doc = "共享状态。\n* PUBLISHED: 发布。模板可用。\n- DRAFT: 草稿。编辑态,仅拥有者可访问。\n- REVIEW:审核态。不可编辑,仅拥有者/审核人员可查看。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_state: Option<SharedConfigSharedState>,
    #[doc = "共享类型。\n* PRIVATE: 私有,仅本租户可访问。\n* PUBLIC: 公开,所有租户可访问。当前仅提供系统资产可公开访问。\n* SHARED:共享,指定租户可访问。拥有者指定租户可访问。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_type: Option<SharedConfigSharedType>,
}
impl From<&SharedConfig> for SharedConfig {
    fn from(value: &SharedConfig) -> Self {
        value.clone()
    }
}
impl SharedConfig {
    pub fn builder() -> builder::SharedConfig {
        Default::default()
    }
}
#[doc = "SharedConfigAllowedProjectIdsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"maxLength\": 64,\n  \"minLength\": 1\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SharedConfigAllowedProjectIdsItem(String);
impl std::ops::Deref for SharedConfigAllowedProjectIdsItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SharedConfigAllowedProjectIdsItem> for String {
    fn from(value: SharedConfigAllowedProjectIdsItem) -> Self {
        value.0
    }
}
impl From<&SharedConfigAllowedProjectIdsItem> for SharedConfigAllowedProjectIdsItem {
    fn from(value: &SharedConfigAllowedProjectIdsItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SharedConfigAllowedProjectIdsItem {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 64usize {
            return Err("longer than 64 characters");
        }
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for SharedConfigAllowedProjectIdsItem {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SharedConfigAllowedProjectIdsItem {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SharedConfigAllowedProjectIdsItem {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for SharedConfigAllowedProjectIdsItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "共享过期时间。空表示永久不过期。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"共享过期时间。空表示永久不过期。\",\n  \"type\": \"string\",\n  \"maxLength\": 20,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SharedConfigExpireTime(String);
impl std::ops::Deref for SharedConfigExpireTime {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SharedConfigExpireTime> for String {
    fn from(value: SharedConfigExpireTime) -> Self {
        value.0
    }
}
impl From<&SharedConfigExpireTime> for SharedConfigExpireTime {
    fn from(value: &SharedConfigExpireTime) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SharedConfigExpireTime {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 20usize {
            return Err("longer than 20 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for SharedConfigExpireTime {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SharedConfigExpireTime {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SharedConfigExpireTime {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for SharedConfigExpireTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "共享状态。\n* PUBLISHED: 发布。模板可用。\n- DRAFT: 草稿。编辑态,仅拥有者可访问。\n- REVIEW:审核态。不可编辑,仅拥有者/审核人员可查看。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"共享状态。\\n* PUBLISHED: 发布。模板可用。\\n- DRAFT: 草稿。编辑态,仅拥有者可访问。\\n- REVIEW:审核态。不可编辑,仅拥有者/审核人员可查看。\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"PUBLISHED\",\n    \"DRAFT\",\n    \"REVIEW\"\n  ],\n  \"maxLength\": 32,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SharedConfigSharedState {
    #[serde(rename = "PUBLISHED")]
    Published,
    #[serde(rename = "DRAFT")]
    Draft,
    #[serde(rename = "REVIEW")]
    Review,
}
impl From<&SharedConfigSharedState> for SharedConfigSharedState {
    fn from(value: &SharedConfigSharedState) -> Self {
        value.clone()
    }
}
impl ToString for SharedConfigSharedState {
    fn to_string(&self) -> String {
        match *self {
            Self::Published => "PUBLISHED".to_string(),
            Self::Draft => "DRAFT".to_string(),
            Self::Review => "REVIEW".to_string(),
        }
    }
}
impl std::str::FromStr for SharedConfigSharedState {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "PUBLISHED" => Ok(Self::Published),
            "DRAFT" => Ok(Self::Draft),
            "REVIEW" => Ok(Self::Review),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for SharedConfigSharedState {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SharedConfigSharedState {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SharedConfigSharedState {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "共享类型。\n* PRIVATE: 私有,仅本租户可访问。\n* PUBLIC: 公开,所有租户可访问。当前仅提供系统资产可公开访问。\n* SHARED:共享,指定租户可访问。拥有者指定租户可访问。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"共享类型。\\n* PRIVATE: 私有,仅本租户可访问。\\n* PUBLIC: 公开,所有租户可访问。当前仅提供系统资产可公开访问。\\n* SHARED:共享,指定租户可访问。拥有者指定租户可访问。\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"PRIVATE\",\n    \"PUBLIC\",\n    \"SHARED\"\n  ],\n  \"maxLength\": 32,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SharedConfigSharedType {
    #[serde(rename = "PRIVATE")]
    Private,
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "SHARED")]
    Shared,
}
impl From<&SharedConfigSharedType> for SharedConfigSharedType {
    fn from(value: &SharedConfigSharedType) -> Self {
        value.clone()
    }
}
impl ToString for SharedConfigSharedType {
    fn to_string(&self) -> String {
        match *self {
            Self::Private => "PRIVATE".to_string(),
            Self::Public => "PUBLIC".to_string(),
            Self::Shared => "SHARED".to_string(),
        }
    }
}
impl std::str::FromStr for SharedConfigSharedType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "PRIVATE" => Ok(Self::Private),
            "PUBLIC" => Ok(Self::Public),
            "SHARED" => Ok(Self::Shared),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for SharedConfigSharedType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SharedConfigSharedType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SharedConfigSharedType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "素材图片图层配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"素材图片图层配置。\",\n  \"type\": \"object\",\n  \"required\": [\n    \"image_url\"\n  ],\n  \"properties\": {\n    \"display_duration\": {\n      \"description\": \"图片显示时长。单位s\\n* 0表示一直显示。\",\n      \"type\": \"integer\",\n      \"maximum\": 3600.0,\n      \"minimum\": 0.0\n    },\n    \"image_url\": {\n      \"description\": \"图片文件的URL。\",\n      \"type\": \"string\",\n      \"maxLength\": 2048,\n      \"minLength\": 0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SmartImageLayerConfig {
    #[doc = "图片显示时长。单位s\n* 0表示一直显示。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_duration: Option<i64>,
    #[doc = "图片文件的URL。"]
    pub image_url: SmartImageLayerConfigImageUrl,
}
impl From<&SmartImageLayerConfig> for SmartImageLayerConfig {
    fn from(value: &SmartImageLayerConfig) -> Self {
        value.clone()
    }
}
impl SmartImageLayerConfig {
    pub fn builder() -> builder::SmartImageLayerConfig {
        Default::default()
    }
}
#[doc = "图片文件的URL。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"图片文件的URL。\",\n  \"type\": \"string\",\n  \"maxLength\": 2048,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SmartImageLayerConfigImageUrl(String);
impl std::ops::Deref for SmartImageLayerConfigImageUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SmartImageLayerConfigImageUrl> for String {
    fn from(value: SmartImageLayerConfigImageUrl) -> Self {
        value.0
    }
}
impl From<&SmartImageLayerConfigImageUrl> for SmartImageLayerConfigImageUrl {
    fn from(value: &SmartImageLayerConfigImageUrl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SmartImageLayerConfigImageUrl {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 2048usize {
            return Err("longer than 2048 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for SmartImageLayerConfigImageUrl {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SmartImageLayerConfigImageUrl {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SmartImageLayerConfigImageUrl {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for SmartImageLayerConfigImageUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "智能图层配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"智能图层配置。\",\n  \"required\": [\n    \"layer_type\",\n    \"position\"\n  ],\n  \"properties\": {\n    \"image_config\": {\n      \"$ref\": \"#/definitions/SmartImageLayerConfig\"\n    },\n    \"layer_type\": {\n      \"description\": \"图层类型。\\n- IMAGE: 素材图片图层\\n- VIDEO: 素材视频图层\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"IMAGE\",\n        \"VIDEO\"\n      ]\n    },\n    \"position\": {\n      \"$ref\": \"#/definitions/LayerPositionConfig\"\n    },\n    \"size\": {\n      \"$ref\": \"#/definitions/LayerSizeConfig\"\n    },\n    \"video_config\": {\n      \"$ref\": \"#/definitions/SmartVideoLayerConfig\"\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SmartLayerConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_config: Option<SmartImageLayerConfig>,
    #[doc = "图层类型。\n- IMAGE: 素材图片图层\n- VIDEO: 素材视频图层"]
    pub layer_type: SmartLayerConfigLayerType,
    pub position: LayerPositionConfig,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<LayerSizeConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_config: Option<SmartVideoLayerConfig>,
}
impl From<&SmartLayerConfig> for SmartLayerConfig {
    fn from(value: &SmartLayerConfig) -> Self {
        value.clone()
    }
}
impl SmartLayerConfig {
    pub fn builder() -> builder::SmartLayerConfig {
        Default::default()
    }
}
#[doc = "图层类型。\n- IMAGE: 素材图片图层\n- VIDEO: 素材视频图层"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"图层类型。\\n- IMAGE: 素材图片图层\\n- VIDEO: 素材视频图层\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"IMAGE\",\n    \"VIDEO\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SmartLayerConfigLayerType {
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "VIDEO")]
    Video,
}
impl From<&SmartLayerConfigLayerType> for SmartLayerConfigLayerType {
    fn from(value: &SmartLayerConfigLayerType) -> Self {
        value.clone()
    }
}
impl ToString for SmartLayerConfigLayerType {
    fn to_string(&self) -> String {
        match *self {
            Self::Image => "IMAGE".to_string(),
            Self::Video => "VIDEO".to_string(),
        }
    }
}
impl std::str::FromStr for SmartLayerConfigLayerType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "IMAGE" => Ok(Self::Image),
            "VIDEO" => Ok(Self::Video),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for SmartLayerConfigLayerType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SmartLayerConfigLayerType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SmartLayerConfigLayerType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "素材视频图层配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"素材视频图层配置。\",\n  \"type\": \"object\",\n  \"required\": [\n    \"video_url\"\n  ],\n  \"properties\": {\n    \"display_duration\": {\n      \"description\": \"视频显示时长。单位s\\n* 0:表示一直显示。\",\n      \"type\": \"integer\",\n      \"maximum\": 3600.0,\n      \"minimum\": 0.0\n    },\n    \"video_cover_url\": {\n      \"description\": \"视频封面文件的URL。\",\n      \"type\": \"string\",\n      \"maxLength\": 2048,\n      \"minLength\": 0\n    },\n    \"video_url\": {\n      \"description\": \"视频文件的URL。\",\n      \"type\": \"string\",\n      \"maxLength\": 2048,\n      \"minLength\": 0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SmartVideoLayerConfig {
    #[doc = "视频显示时长。单位s\n* 0:表示一直显示。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_duration: Option<i64>,
    #[doc = "视频封面文件的URL。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_cover_url: Option<SmartVideoLayerConfigVideoCoverUrl>,
    #[doc = "视频文件的URL。"]
    pub video_url: SmartVideoLayerConfigVideoUrl,
}
impl From<&SmartVideoLayerConfig> for SmartVideoLayerConfig {
    fn from(value: &SmartVideoLayerConfig) -> Self {
        value.clone()
    }
}
impl SmartVideoLayerConfig {
    pub fn builder() -> builder::SmartVideoLayerConfig {
        Default::default()
    }
}
#[doc = "视频封面文件的URL。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"视频封面文件的URL。\",\n  \"type\": \"string\",\n  \"maxLength\": 2048,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SmartVideoLayerConfigVideoCoverUrl(String);
impl std::ops::Deref for SmartVideoLayerConfigVideoCoverUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SmartVideoLayerConfigVideoCoverUrl> for String {
    fn from(value: SmartVideoLayerConfigVideoCoverUrl) -> Self {
        value.0
    }
}
impl From<&SmartVideoLayerConfigVideoCoverUrl> for SmartVideoLayerConfigVideoCoverUrl {
    fn from(value: &SmartVideoLayerConfigVideoCoverUrl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SmartVideoLayerConfigVideoCoverUrl {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 2048usize {
            return Err("longer than 2048 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for SmartVideoLayerConfigVideoCoverUrl {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SmartVideoLayerConfigVideoCoverUrl {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SmartVideoLayerConfigVideoCoverUrl {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for SmartVideoLayerConfigVideoCoverUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "视频文件的URL。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"视频文件的URL。\",\n  \"type\": \"string\",\n  \"maxLength\": 2048,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SmartVideoLayerConfigVideoUrl(String);
impl std::ops::Deref for SmartVideoLayerConfigVideoUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SmartVideoLayerConfigVideoUrl> for String {
    fn from(value: SmartVideoLayerConfigVideoUrl) -> Self {
        value.0
    }
}
impl From<&SmartVideoLayerConfigVideoUrl> for SmartVideoLayerConfigVideoUrl {
    fn from(value: &SmartVideoLayerConfigVideoUrl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SmartVideoLayerConfigVideoUrl {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 2048usize {
            return Err("longer than 2048 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for SmartVideoLayerConfigVideoUrl {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SmartVideoLayerConfigVideoUrl {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SmartVideoLayerConfigVideoUrl {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for SmartVideoLayerConfigVideoUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "字幕配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"字幕配置。\",\n  \"properties\": {\n    \"dx\": {\n      \"description\": \"字幕左上角像素点坐标。\\n\\n> *横屏(16:9)视频像素为1920x1080;竖屏(9:16)视频像素为1080x1920。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 1920.0,\n      \"minimum\": 0.0,\n      \"example\": 256\n    },\n    \"dy\": {\n      \"description\": \"字幕左上角像素点坐标。\\n\\n> *横屏(16:9)视频像素为1920x1080;竖屏(9:16)视频像素为1080x1920。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 1920.0,\n      \"minimum\": 0.0\n    },\n    \"font_name\": {\n      \"description\": \"字体。当前支持的字体:\\n* HarmonyOS_Sans_SC_Black:鸿蒙粗体\\n* HarmonyOS_Sans_SC_Regular:鸿蒙常规\\n* HarmonyOS_Sans_SC_Thin:鸿蒙细体\",\n      \"default\": \"HarmonyOS_Sans_SC_Black\",\n      \"type\": \"string\",\n      \"maxLength\": 64,\n      \"minLength\": 0\n    },\n    \"font_size\": {\n      \"description\": \"字体大小。\\n\\n取值范围:[4, 120]\",\n      \"default\": 16,\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 120.0,\n      \"minimum\": 0.0\n    },\n    \"h\": {\n      \"description\": \"字幕框高度\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 1920.0,\n      \"minimum\": 0.0\n    },\n    \"w\": {\n      \"description\": \"字幕框宽度\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 1920.0,\n      \"minimum\": 0.0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubtitleConfig {
    #[doc = "字幕左上角像素点坐标。\n\n> *横屏(16:9)视频像素为1920x1080;竖屏(9:16)视频像素为1080x1920。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dx: Option<i64>,
    #[doc = "字幕左上角像素点坐标。\n\n> *横屏(16:9)视频像素为1920x1080;竖屏(9:16)视频像素为1080x1920。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dy: Option<i64>,
    #[doc = "字体。当前支持的字体:\n* HarmonyOS_Sans_SC_Black:鸿蒙粗体\n* HarmonyOS_Sans_SC_Regular:鸿蒙常规\n* HarmonyOS_Sans_SC_Thin:鸿蒙细体"]
    #[serde(default = "defaults::subtitle_config_font_name")]
    pub font_name: SubtitleConfigFontName,
    #[doc = "字体大小。\n\n取值范围:[4, 120]"]
    #[serde(default = "defaults::default_u64::<i64, 16>")]
    pub font_size: i64,
    #[doc = "字幕框高度"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h: Option<i64>,
    #[doc = "字幕框宽度"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<i64>,
}
impl From<&SubtitleConfig> for SubtitleConfig {
    fn from(value: &SubtitleConfig) -> Self {
        value.clone()
    }
}
impl SubtitleConfig {
    pub fn builder() -> builder::SubtitleConfig {
        Default::default()
    }
}
#[doc = "字体。当前支持的字体:\n* HarmonyOS_Sans_SC_Black:鸿蒙粗体\n* HarmonyOS_Sans_SC_Regular:鸿蒙常规\n* HarmonyOS_Sans_SC_Thin:鸿蒙细体"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"字体。当前支持的字体:\\n* HarmonyOS_Sans_SC_Black:鸿蒙粗体\\n* HarmonyOS_Sans_SC_Regular:鸿蒙常规\\n* HarmonyOS_Sans_SC_Thin:鸿蒙细体\",\n  \"default\": \"HarmonyOS_Sans_SC_Black\",\n  \"type\": \"string\",\n  \"maxLength\": 64,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SubtitleConfigFontName(String);
impl std::ops::Deref for SubtitleConfigFontName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<SubtitleConfigFontName> for String {
    fn from(value: SubtitleConfigFontName) -> Self {
        value.0
    }
}
impl From<&SubtitleConfigFontName> for SubtitleConfigFontName {
    fn from(value: &SubtitleConfigFontName) -> Self {
        value.clone()
    }
}
impl Default for SubtitleConfigFontName {
    fn default() -> Self {
        SubtitleConfigFontName("HarmonyOS_Sans_SC_Black".to_string())
    }
}
impl std::str::FromStr for SubtitleConfigFontName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 64usize {
            return Err("longer than 64 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for SubtitleConfigFontName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SubtitleConfigFontName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SubtitleConfigFontName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for SubtitleConfigFontName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "台词脚本。\n> * 最长2000个字符,不含SSML标签字符数。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"台词脚本。\\n> * 最长2000个字符,不含SSML标签字符数。\",\n  \"required\": [\n    \"text\"\n  ],\n  \"properties\": {\n    \"text\": {\n      \"description\": \"台词脚本。\\n\\n支持两种模式,纯文本模式和标签模式。\\n\\n### 纯文本模式\\n纯文本模式,使用方法,如“大家好,我是人工智大家,是个虚拟主播”\\n\\n### 标签模式\\n标签模式的定义使用SSML(Speech Synthesis Markup Language)标记语言。\\n\\n- **\\\\<speak>**\\n\\n  \\\\<speak>标签是所有文本的根节点。一切需要调用SSML标签的文本都要包含在\\\\<speak> \\\\</speak>对中。\\n\\n- **\\\\<emotion>**\\n\\n  \\\\<emotion>标签是情感标签。对指定一句或多句话生效,标签开始在句子起始位置,标签关闭在句子结尾。用法:\\\\<emotion type=\\\"情感标签\\\">。type可取值包括:HAPPY、SAD、CALM、ANGER\\n\\n- **\\\\<insert-action>**\\n\\n  \\\\<insert-action>标签是动作标签。在文本的指定位置插入动作。用法:\\\\<insert-action id=\\\"动作资产ID\\\" name=\\\"动作名称\\\" tag=\\\"动作标识\\\"/>。动作资产信息通过资产库接口查询获取。\\n\\n- **\\\\<break>**\\n\\n   \\\\<break>标签是停顿标签。在文本的指定位置插入停顿。用法:\\\\<break time=\\\"停顿时长\\\"/>。停顿时长单位是毫秒,最小值200毫秒。\\n\\n- **\\\\<phoneme>**\\n\\n   \\\\<phoneme>标签是多音字标签。多音字标签可以指定单个汉字的读音。标签起始和关闭之间只能包含1个汉字。属性可取值为汉语拼音,声调用1、2、3、4表示。用法:\\\\<phoneme ph=\\\"拼音\\\"/>字\\\\</phoneme>。\\n\\n\\n> * 举例:\\\\<speak> \\\\<emotion type=\\\\\\\"HAPPY\\\\\\\">\\\\<insert-action id=\\\"2692ea5d095caaafcfed21dc4590b701\\\" name=\\\"双手指尖交触\\\" tag=\\\"system_female_animation_0026\\\"\\\\/>大家好,\\\\<break time=\\\"200ms\\\"\\\\/>我是MetaStudio制作的人工智能数字人。\\\\</emotion>我带大家\\\\<phoneme ph=\\\"liao3\\\">了\\\\</phoneme>解MetaStudio。\\\\</speak>\\n> * 分身数字人视频制作仅break和phoneme标签生效。\",\n      \"type\": \"string\",\n      \"maxLength\": 131072,\n      \"minLength\": 1,\n      \"x-annotations\": \"@TextLength\"\n    }\n  },\n  \"x-imports\": [\n    \"com.huawei.hdh.mps.annotations.TextLength\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextConfig {
    #[doc = "台词脚本。\n\n支持两种模式,纯文本模式和标签模式。\n\n### 纯文本模式\n纯文本模式,使用方法,如“大家好,我是人工智大家,是个虚拟主播”\n\n### 标签模式\n标签模式的定义使用SSML(Speech Synthesis Markup Language)标记语言。\n\n- **\\<speak>**\n\n  \\<speak>标签是所有文本的根节点。一切需要调用SSML标签的文本都要包含在\\<speak> \\</speak>对中。\n\n- **\\<emotion>**\n\n  \\<emotion>标签是情感标签。对指定一句或多句话生效,标签开始在句子起始位置,标签关闭在句子结尾。用法:\\<emotion type=\"情感标签\">。type可取值包括:HAPPY、SAD、CALM、ANGER\n\n- **\\<insert-action>**\n\n  \\<insert-action>标签是动作标签。在文本的指定位置插入动作。用法:\\<insert-action id=\"动作资产ID\" name=\"动作名称\" tag=\"动作标识\"/>。动作资产信息通过资产库接口查询获取。\n\n- **\\<break>**\n\n   \\<break>标签是停顿标签。在文本的指定位置插入停顿。用法:\\<break time=\"停顿时长\"/>。停顿时长单位是毫秒,最小值200毫秒。\n\n- **\\<phoneme>**\n\n   \\<phoneme>标签是多音字标签。多音字标签可以指定单个汉字的读音。标签起始和关闭之间只能包含1个汉字。属性可取值为汉语拼音,声调用1、2、3、4表示。用法:\\<phoneme ph=\"拼音\"/>字\\</phoneme>。\n\n\n> * 举例:\\<speak> \\<emotion type=\\\"HAPPY\\\">\\<insert-action id=\"2692ea5d095caaafcfed21dc4590b701\" name=\"双手指尖交触\" tag=\"system_female_animation_0026\"\\/>大家好,\\<break time=\"200ms\"\\/>我是MetaStudio制作的人工智能数字人。\\</emotion>我带大家\\<phoneme ph=\"liao3\">了\\</phoneme>解MetaStudio。\\</speak>\n> * 分身数字人视频制作仅break和phoneme标签生效。"]
    pub text: TextConfigText,
}
impl From<&TextConfig> for TextConfig {
    fn from(value: &TextConfig) -> Self {
        value.clone()
    }
}
impl TextConfig {
    pub fn builder() -> builder::TextConfig {
        Default::default()
    }
}
#[doc = "台词脚本。\n\n支持两种模式,纯文本模式和标签模式。\n\n### 纯文本模式\n纯文本模式,使用方法,如“大家好,我是人工智大家,是个虚拟主播”\n\n### 标签模式\n标签模式的定义使用SSML(Speech Synthesis Markup Language)标记语言。\n\n- **\\<speak>**\n\n  \\<speak>标签是所有文本的根节点。一切需要调用SSML标签的文本都要包含在\\<speak> \\</speak>对中。\n\n- **\\<emotion>**\n\n  \\<emotion>标签是情感标签。对指定一句或多句话生效,标签开始在句子起始位置,标签关闭在句子结尾。用法:\\<emotion type=\"情感标签\">。type可取值包括:HAPPY、SAD、CALM、ANGER\n\n- **\\<insert-action>**\n\n  \\<insert-action>标签是动作标签。在文本的指定位置插入动作。用法:\\<insert-action id=\"动作资产ID\" name=\"动作名称\" tag=\"动作标识\"/>。动作资产信息通过资产库接口查询获取。\n\n- **\\<break>**\n\n   \\<break>标签是停顿标签。在文本的指定位置插入停顿。用法:\\<break time=\"停顿时长\"/>。停顿时长单位是毫秒,最小值200毫秒。\n\n- **\\<phoneme>**\n\n   \\<phoneme>标签是多音字标签。多音字标签可以指定单个汉字的读音。标签起始和关闭之间只能包含1个汉字。属性可取值为汉语拼音,声调用1、2、3、4表示。用法:\\<phoneme ph=\"拼音\"/>字\\</phoneme>。\n\n\n> * 举例:\\<speak> \\<emotion type=\\\"HAPPY\\\">\\<insert-action id=\"2692ea5d095caaafcfed21dc4590b701\" name=\"双手指尖交触\" tag=\"system_female_animation_0026\"\\/>大家好,\\<break time=\"200ms\"\\/>我是MetaStudio制作的人工智能数字人。\\</emotion>我带大家\\<phoneme ph=\"liao3\">了\\</phoneme>解MetaStudio。\\</speak>\n> * 分身数字人视频制作仅break和phoneme标签生效。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"台词脚本。\\n\\n支持两种模式,纯文本模式和标签模式。\\n\\n### 纯文本模式\\n纯文本模式,使用方法,如“大家好,我是人工智大家,是个虚拟主播”\\n\\n### 标签模式\\n标签模式的定义使用SSML(Speech Synthesis Markup Language)标记语言。\\n\\n- **\\\\<speak>**\\n\\n  \\\\<speak>标签是所有文本的根节点。一切需要调用SSML标签的文本都要包含在\\\\<speak> \\\\</speak>对中。\\n\\n- **\\\\<emotion>**\\n\\n  \\\\<emotion>标签是情感标签。对指定一句或多句话生效,标签开始在句子起始位置,标签关闭在句子结尾。用法:\\\\<emotion type=\\\"情感标签\\\">。type可取值包括:HAPPY、SAD、CALM、ANGER\\n\\n- **\\\\<insert-action>**\\n\\n  \\\\<insert-action>标签是动作标签。在文本的指定位置插入动作。用法:\\\\<insert-action id=\\\"动作资产ID\\\" name=\\\"动作名称\\\" tag=\\\"动作标识\\\"/>。动作资产信息通过资产库接口查询获取。\\n\\n- **\\\\<break>**\\n\\n   \\\\<break>标签是停顿标签。在文本的指定位置插入停顿。用法:\\\\<break time=\\\"停顿时长\\\"/>。停顿时长单位是毫秒,最小值200毫秒。\\n\\n- **\\\\<phoneme>**\\n\\n   \\\\<phoneme>标签是多音字标签。多音字标签可以指定单个汉字的读音。标签起始和关闭之间只能包含1个汉字。属性可取值为汉语拼音,声调用1、2、3、4表示。用法:\\\\<phoneme ph=\\\"拼音\\\"/>字\\\\</phoneme>。\\n\\n\\n> * 举例:\\\\<speak> \\\\<emotion type=\\\\\\\"HAPPY\\\\\\\">\\\\<insert-action id=\\\"2692ea5d095caaafcfed21dc4590b701\\\" name=\\\"双手指尖交触\\\" tag=\\\"system_female_animation_0026\\\"\\\\/>大家好,\\\\<break time=\\\"200ms\\\"\\\\/>我是MetaStudio制作的人工智能数字人。\\\\</emotion>我带大家\\\\<phoneme ph=\\\"liao3\\\">了\\\\</phoneme>解MetaStudio。\\\\</speak>\\n> * 分身数字人视频制作仅break和phoneme标签生效。\",\n  \"type\": \"string\",\n  \"maxLength\": 131072,\n  \"minLength\": 1,\n  \"x-annotations\": \"@TextLength\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TextConfigText(String);
impl std::ops::Deref for TextConfigText {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TextConfigText> for String {
    fn from(value: TextConfigText) -> Self {
        value.0
    }
}
impl From<&TextConfigText> for TextConfigText {
    fn from(value: &TextConfigText) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for TextConfigText {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 131072usize {
            return Err("longer than 131072 characters");
        }
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for TextConfigText {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TextConfigText {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TextConfigText {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for TextConfigText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "素材文字图层配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"素材文字图层配置。\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"font_color\": {\n      \"description\": \"字体颜色。RGB颜色值。\",\n      \"default\": \"16777215\",\n      \"type\": \"string\",\n      \"maxLength\": 16,\n      \"minLength\": 0\n    },\n    \"font_name\": {\n      \"description\": \"字体。当前支持的字体:\\n* HarmonyOS_Sans_SC_Black:鸿蒙粗体\\n* HarmonyOS_Sans_SC_Regular:鸿蒙常规\\n* HarmonyOS_Sans_SC_Thin:鸿蒙细体\\n* fzyouh:方正瘦体\",\n      \"default\": \"HarmonyOS_Sans_SC_Black\",\n      \"type\": \"string\",\n      \"maxLength\": 64,\n      \"minLength\": 0\n    },\n    \"font_size\": {\n      \"description\": \"字体大小(像素)。\\n\\n取值范围:[4, 120]\",\n      \"default\": 16,\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 120.0,\n      \"minimum\": 0.0\n    },\n    \"text_context\": {\n      \"description\": \"文字图层的文本,内容需做Base64编码。\\n\\n示例:若想添加文字水印“测试文字水印”,那么text_context的值为:5rWL6K+V5paH5a2X5rC05Y2w\",\n      \"type\": \"string\",\n      \"maxLength\": 1024,\n      \"minLength\": 0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextLayerConfig {
    #[doc = "字体颜色。RGB颜色值。"]
    #[serde(default = "defaults::text_layer_config_font_color")]
    pub font_color: TextLayerConfigFontColor,
    #[doc = "字体。当前支持的字体:\n* HarmonyOS_Sans_SC_Black:鸿蒙粗体\n* HarmonyOS_Sans_SC_Regular:鸿蒙常规\n* HarmonyOS_Sans_SC_Thin:鸿蒙细体\n* fzyouh:方正瘦体"]
    #[serde(default = "defaults::text_layer_config_font_name")]
    pub font_name: TextLayerConfigFontName,
    #[doc = "字体大小(像素)。\n\n取值范围:[4, 120]"]
    #[serde(default = "defaults::default_u64::<i64, 16>")]
    pub font_size: i64,
    #[doc = "文字图层的文本,内容需做Base64编码。\n\n示例:若想添加文字水印“测试文字水印”,那么text_context的值为:5rWL6K+V5paH5a2X5rC05Y2w"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_context: Option<TextLayerConfigTextContext>,
}
impl From<&TextLayerConfig> for TextLayerConfig {
    fn from(value: &TextLayerConfig) -> Self {
        value.clone()
    }
}
impl TextLayerConfig {
    pub fn builder() -> builder::TextLayerConfig {
        Default::default()
    }
}
#[doc = "字体颜色。RGB颜色值。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"字体颜色。RGB颜色值。\",\n  \"default\": \"16777215\",\n  \"type\": \"string\",\n  \"maxLength\": 16,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TextLayerConfigFontColor(String);
impl std::ops::Deref for TextLayerConfigFontColor {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TextLayerConfigFontColor> for String {
    fn from(value: TextLayerConfigFontColor) -> Self {
        value.0
    }
}
impl From<&TextLayerConfigFontColor> for TextLayerConfigFontColor {
    fn from(value: &TextLayerConfigFontColor) -> Self {
        value.clone()
    }
}
impl Default for TextLayerConfigFontColor {
    fn default() -> Self {
        TextLayerConfigFontColor("16777215".to_string())
    }
}
impl std::str::FromStr for TextLayerConfigFontColor {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 16usize {
            return Err("longer than 16 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for TextLayerConfigFontColor {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TextLayerConfigFontColor {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TextLayerConfigFontColor {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for TextLayerConfigFontColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "字体。当前支持的字体:\n* HarmonyOS_Sans_SC_Black:鸿蒙粗体\n* HarmonyOS_Sans_SC_Regular:鸿蒙常规\n* HarmonyOS_Sans_SC_Thin:鸿蒙细体\n* fzyouh:方正瘦体"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"字体。当前支持的字体:\\n* HarmonyOS_Sans_SC_Black:鸿蒙粗体\\n* HarmonyOS_Sans_SC_Regular:鸿蒙常规\\n* HarmonyOS_Sans_SC_Thin:鸿蒙细体\\n* fzyouh:方正瘦体\",\n  \"default\": \"HarmonyOS_Sans_SC_Black\",\n  \"type\": \"string\",\n  \"maxLength\": 64,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TextLayerConfigFontName(String);
impl std::ops::Deref for TextLayerConfigFontName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TextLayerConfigFontName> for String {
    fn from(value: TextLayerConfigFontName) -> Self {
        value.0
    }
}
impl From<&TextLayerConfigFontName> for TextLayerConfigFontName {
    fn from(value: &TextLayerConfigFontName) -> Self {
        value.clone()
    }
}
impl Default for TextLayerConfigFontName {
    fn default() -> Self {
        TextLayerConfigFontName("HarmonyOS_Sans_SC_Black".to_string())
    }
}
impl std::str::FromStr for TextLayerConfigFontName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 64usize {
            return Err("longer than 64 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for TextLayerConfigFontName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TextLayerConfigFontName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TextLayerConfigFontName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for TextLayerConfigFontName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "文字图层的文本,内容需做Base64编码。\n\n示例:若想添加文字水印“测试文字水印”,那么text_context的值为:5rWL6K+V5paH5a2X5rC05Y2w"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"文字图层的文本,内容需做Base64编码。\\n\\n示例:若想添加文字水印“测试文字水印”,那么text_context的值为:5rWL6K+V5paH5a2X5rC05Y2w\",\n  \"type\": \"string\",\n  \"maxLength\": 1024,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TextLayerConfigTextContext(String);
impl std::ops::Deref for TextLayerConfigTextContext {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TextLayerConfigTextContext> for String {
    fn from(value: TextLayerConfigTextContext) -> Self {
        value.0
    }
}
impl From<&TextLayerConfigTextContext> for TextLayerConfigTextContext {
    fn from(value: &TextLayerConfigTextContext) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for TextLayerConfigTextContext {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 1024usize {
            return Err("longer than 1024 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for TextLayerConfigTextContext {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TextLayerConfigTextContext {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TextLayerConfigTextContext {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for TextLayerConfigTextContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "触发器处理"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"触发器处理\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"layer_config\": {\n      \"$ref\": \"#/definitions/SmartLayerConfig\"\n    },\n    \"reply_audios\": {\n      \"description\": \"回复音频集。填写audio_url。\",\n      \"type\": \"array\",\n      \"items\": {\n        \"$ref\": \"#/definitions/ReplyAudioInfo\"\n      },\n      \"maxItems\": 5,\n      \"minItems\": 0\n    },\n    \"reply_mode\": {\n      \"description\": \"回复类型。\\n* SYSTEM_REPLY:系统自动回复设置的话术。\\n* CALLBACK:回调给其他服务,携带设置的话术。\\n* SHOW_LAYER: 显示叠加图层,不影响话术。\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"SYSTEM_REPLY\",\n        \"CALLBACK\",\n        \"SHOW_LAYER\"\n      ],\n      \"maxLength\": 16,\n      \"minLength\": 0\n    },\n    \"reply_order\": {\n      \"description\": \"回复次序\\n- RANDOM:随机\\n- ORDER:顺序循环\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"RANDOM\",\n        \"ORDER\"\n      ],\n      \"maxLength\": 32,\n      \"minLength\": 0\n    },\n    \"reply_texts\": {\n      \"description\": \"回复话术集\",\n      \"type\": \"array\",\n      \"items\": {\n        \"type\": \"string\",\n        \"maxLength\": 1024,\n        \"minLength\": 0\n      },\n      \"maxItems\": 5,\n      \"minItems\": 0\n    },\n    \"time_window\": {\n      \"description\": \"处理抑制时长。单位秒。\\n-1 表示整场直播\\n0 表示无抑制,每次都触发\",\n      \"type\": \"integer\",\n      \"maximum\": 7200.0,\n      \"minimum\": -1.0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TriggerProcess {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer_config: Option<SmartLayerConfig>,
    #[doc = "回复音频集。填写audio_url。"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reply_audios: Vec<ReplyAudioInfo>,
    #[doc = "回复类型。\n* SYSTEM_REPLY:系统自动回复设置的话术。\n* CALLBACK:回调给其他服务,携带设置的话术。\n* SHOW_LAYER: 显示叠加图层,不影响话术。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_mode: Option<TriggerProcessReplyMode>,
    #[doc = "回复次序\n- RANDOM:随机\n- ORDER:顺序循环"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_order: Option<TriggerProcessReplyOrder>,
    #[doc = "回复话术集"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reply_texts: Vec<TriggerProcessReplyTextsItem>,
    #[doc = "处理抑制时长。单位秒。\n-1 表示整场直播\n0 表示无抑制,每次都触发"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_window: Option<i64>,
}
impl From<&TriggerProcess> for TriggerProcess {
    fn from(value: &TriggerProcess) -> Self {
        value.clone()
    }
}
impl TriggerProcess {
    pub fn builder() -> builder::TriggerProcess {
        Default::default()
    }
}
#[doc = "回复类型。\n* SYSTEM_REPLY:系统自动回复设置的话术。\n* CALLBACK:回调给其他服务,携带设置的话术。\n* SHOW_LAYER: 显示叠加图层,不影响话术。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"回复类型。\\n* SYSTEM_REPLY:系统自动回复设置的话术。\\n* CALLBACK:回调给其他服务,携带设置的话术。\\n* SHOW_LAYER: 显示叠加图层,不影响话术。\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"SYSTEM_REPLY\",\n    \"CALLBACK\",\n    \"SHOW_LAYER\"\n  ],\n  \"maxLength\": 16,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TriggerProcessReplyMode {
    #[serde(rename = "SYSTEM_REPLY")]
    SystemReply,
    #[serde(rename = "CALLBACK")]
    Callback,
    #[serde(rename = "SHOW_LAYER")]
    ShowLayer,
}
impl From<&TriggerProcessReplyMode> for TriggerProcessReplyMode {
    fn from(value: &TriggerProcessReplyMode) -> Self {
        value.clone()
    }
}
impl ToString for TriggerProcessReplyMode {
    fn to_string(&self) -> String {
        match *self {
            Self::SystemReply => "SYSTEM_REPLY".to_string(),
            Self::Callback => "CALLBACK".to_string(),
            Self::ShowLayer => "SHOW_LAYER".to_string(),
        }
    }
}
impl std::str::FromStr for TriggerProcessReplyMode {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "SYSTEM_REPLY" => Ok(Self::SystemReply),
            "CALLBACK" => Ok(Self::Callback),
            "SHOW_LAYER" => Ok(Self::ShowLayer),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for TriggerProcessReplyMode {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TriggerProcessReplyMode {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TriggerProcessReplyMode {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "回复次序\n- RANDOM:随机\n- ORDER:顺序循环"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"回复次序\\n- RANDOM:随机\\n- ORDER:顺序循环\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"RANDOM\",\n    \"ORDER\"\n  ],\n  \"maxLength\": 32,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TriggerProcessReplyOrder {
    #[serde(rename = "RANDOM")]
    Random,
    #[serde(rename = "ORDER")]
    Order,
}
impl From<&TriggerProcessReplyOrder> for TriggerProcessReplyOrder {
    fn from(value: &TriggerProcessReplyOrder) -> Self {
        value.clone()
    }
}
impl ToString for TriggerProcessReplyOrder {
    fn to_string(&self) -> String {
        match *self {
            Self::Random => "RANDOM".to_string(),
            Self::Order => "ORDER".to_string(),
        }
    }
}
impl std::str::FromStr for TriggerProcessReplyOrder {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "RANDOM" => Ok(Self::Random),
            "ORDER" => Ok(Self::Order),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for TriggerProcessReplyOrder {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TriggerProcessReplyOrder {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TriggerProcessReplyOrder {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "TriggerProcessReplyTextsItem"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": \"string\",\n  \"maxLength\": 1024,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TriggerProcessReplyTextsItem(String);
impl std::ops::Deref for TriggerProcessReplyTextsItem {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<TriggerProcessReplyTextsItem> for String {
    fn from(value: TriggerProcessReplyTextsItem) -> Self {
        value.0
    }
}
impl From<&TriggerProcessReplyTextsItem> for TriggerProcessReplyTextsItem {
    fn from(value: &TriggerProcessReplyTextsItem) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for TriggerProcessReplyTextsItem {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 1024usize {
            return Err("longer than 1024 characters");
        }
        if value.len() < 0usize {
            return Err("shorter than 0 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for TriggerProcessReplyTextsItem {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TriggerProcessReplyTextsItem {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TriggerProcessReplyTextsItem {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for TriggerProcessReplyTextsItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "视频输出配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"视频输出配置。\",\n  \"type\": \"object\",\n  \"required\": [\n    \"bitrate\",\n    \"codec\",\n    \"height\",\n    \"width\"\n  ],\n  \"properties\": {\n    \"bitrate\": {\n      \"description\": \"输出平均码率。\\n\\n单位:kbps。\\n\\n最小值40,最大值30000。\\n> * 分身数字人视频制作采用质量优先,可能会超过设置的码率。\\n> * 分身数字人直播码率范围[1000, 8000]。\",\n      \"type\": \"integer\",\n      \"maximum\": 30000.0,\n      \"minimum\": 40.0\n    },\n    \"clip_mode\": {\n      \"description\": \"输出视频的剪辑方式。默认值RESIZE。\\n* RESIZE:视频缩放。\\n* CROP:视频裁剪。\",\n      \"default\": \"RESIZE\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"RESIZE\",\n        \"CROP\"\n      ]\n    },\n    \"codec\": {\n      \"description\": \"视频编码格式及视频文件格式。\\n* H264: h264编码,输出mp4文件\\n* VP8:vp8编码,输出webm文件\",\n      \"type\": \"string\",\n      \"enum\": [\n        \"H264\",\n        \"VP8\",\n        \"VP9\"\n      ]\n    },\n    \"dx\": {\n      \"description\": \"裁剪视频左上角像素点坐标。\\n\\nclip_mode= CROP时生效。\\n> *横屏(16:9)视频像素为1920x1080;竖屏(9:16)视频像素为1080x1920。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 3840.0,\n      \"minimum\": -1920.0,\n      \"example\": 256\n    },\n    \"dy\": {\n      \"description\": \"裁剪视频左上角像素点坐标。\\n\\nclip_mode= CROP时生效。\\n> *横屏(16:9)视频像素为1920x1080;竖屏(9:16)视频像素为1080x1920。\",\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 3840.0,\n      \"minimum\": -1920.0\n    },\n    \"frame_rate\": {\n      \"description\": \"帧率。\\n\\n单位:FPS。\\n> *  分身数字人视频固定25FPS。\",\n      \"default\": \"25\",\n      \"type\": \"string\",\n      \"format\": \"string\",\n      \"enum\": [\n        \"24\",\n        \"25\",\n        \"30\"\n      ],\n      \"maxLength\": 10,\n      \"minLength\": 0\n    },\n    \"height\": {\n      \"description\": \"视频高度。\\n\\n单位:像素。\\n\\n最小值320,最大值2560。\\n> * clip_mode=RESIZE时,当前支持1920x1080、1080x1920、1280x720、720x1280、3840x2160、2160x3840六种分辨率分辨率。\\n> * clip_mode=CROP,裁剪后视频,(dx,dy)为原点,保留视频像高度为height。\\n> * 分身数字人直播目前只支持1080x1920。\",\n      \"type\": \"integer\",\n      \"maximum\": 3840.0,\n      \"minimum\": 0.0\n    },\n    \"is_subtitle_enable\": {\n      \"description\": \"输出的视频是否带字幕。默认false。\\n> true: 打开字幕\\n> false: 关闭字幕\",\n      \"default\": false,\n      \"type\": \"boolean\",\n      \"enum\": [\n        true,\n        false\n      ],\n      \"example\": true\n    },\n    \"subtitle_config\": {\n      \"$ref\": \"#/definitions/SubtitleConfig\"\n    },\n    \"width\": {\n      \"description\": \"视频宽度。\\n\\n单位:像素。\\n\\n最小值320,最大值2560。\\n> * clip_mode=RESIZE时,当前支持1920x1080、1080x1920、1280x720、720x1280、3840x2160、2160x3840六种分辨率。4K分辨率视频需要分身数字人模型支持4K的情况下才能使用。\\n> * clip_mode=CROP,裁剪后视频,(dx,dy)为原点,保留视频像宽度为width。\\n> * 分身数字人直播目前只支持1080x1920。\",\n      \"type\": \"integer\",\n      \"maximum\": 3840.0,\n      \"minimum\": 0.0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VideoConfig {
    #[doc = "输出平均码率。\n\n单位:kbps。\n\n最小值40,最大值30000。\n> * 分身数字人视频制作采用质量优先,可能会超过设置的码率。\n> * 分身数字人直播码率范围[1000, 8000]。"]
    pub bitrate: i64,
    #[doc = "输出视频的剪辑方式。默认值RESIZE。\n* RESIZE:视频缩放。\n* CROP:视频裁剪。"]
    #[serde(default = "defaults::video_config_clip_mode")]
    pub clip_mode: VideoConfigClipMode,
    #[doc = "视频编码格式及视频文件格式。\n* H264: h264编码,输出mp4文件\n* VP8:vp8编码,输出webm文件"]
    pub codec: VideoConfigCodec,
    #[doc = "裁剪视频左上角像素点坐标。\n\nclip_mode= CROP时生效。\n> *横屏(16:9)视频像素为1920x1080;竖屏(9:16)视频像素为1080x1920。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dx: Option<i64>,
    #[doc = "裁剪视频左上角像素点坐标。\n\nclip_mode= CROP时生效。\n> *横屏(16:9)视频像素为1920x1080;竖屏(9:16)视频像素为1080x1920。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dy: Option<i64>,
    #[doc = "帧率。\n\n单位:FPS。\n> *  分身数字人视频固定25FPS。"]
    #[serde(default = "defaults::video_config_frame_rate")]
    pub frame_rate: VideoConfigFrameRate,
    #[doc = "视频高度。\n\n单位:像素。\n\n最小值320,最大值2560。\n> * clip_mode=RESIZE时,当前支持1920x1080、1080x1920、1280x720、720x1280、3840x2160、2160x3840六种分辨率分辨率。\n> * clip_mode=CROP,裁剪后视频,(dx,dy)为原点,保留视频像高度为height。\n> * 分身数字人直播目前只支持1080x1920。"]
    pub height: i64,
    #[doc = "输出的视频是否带字幕。默认false。\n> true: 打开字幕\n> false: 关闭字幕"]
    #[serde(default)]
    pub is_subtitle_enable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle_config: Option<SubtitleConfig>,
    #[doc = "视频宽度。\n\n单位:像素。\n\n最小值320,最大值2560。\n> * clip_mode=RESIZE时,当前支持1920x1080、1080x1920、1280x720、720x1280、3840x2160、2160x3840六种分辨率。4K分辨率视频需要分身数字人模型支持4K的情况下才能使用。\n> * clip_mode=CROP,裁剪后视频,(dx,dy)为原点,保留视频像宽度为width。\n> * 分身数字人直播目前只支持1080x1920。"]
    pub width: i64,
}
impl From<&VideoConfig> for VideoConfig {
    fn from(value: &VideoConfig) -> Self {
        value.clone()
    }
}
impl VideoConfig {
    pub fn builder() -> builder::VideoConfig {
        Default::default()
    }
}
#[doc = "输出视频的剪辑方式。默认值RESIZE。\n* RESIZE:视频缩放。\n* CROP:视频裁剪。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"输出视频的剪辑方式。默认值RESIZE。\\n* RESIZE:视频缩放。\\n* CROP:视频裁剪。\",\n  \"default\": \"RESIZE\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"RESIZE\",\n    \"CROP\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VideoConfigClipMode {
    #[serde(rename = "RESIZE")]
    Resize,
    #[serde(rename = "CROP")]
    Crop,
}
impl From<&VideoConfigClipMode> for VideoConfigClipMode {
    fn from(value: &VideoConfigClipMode) -> Self {
        value.clone()
    }
}
impl ToString for VideoConfigClipMode {
    fn to_string(&self) -> String {
        match *self {
            Self::Resize => "RESIZE".to_string(),
            Self::Crop => "CROP".to_string(),
        }
    }
}
impl std::str::FromStr for VideoConfigClipMode {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "RESIZE" => Ok(Self::Resize),
            "CROP" => Ok(Self::Crop),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for VideoConfigClipMode {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for VideoConfigClipMode {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for VideoConfigClipMode {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for VideoConfigClipMode {
    fn default() -> Self {
        VideoConfigClipMode::Resize
    }
}
#[doc = "视频编码格式及视频文件格式。\n* H264: h264编码,输出mp4文件\n* VP8:vp8编码,输出webm文件"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"视频编码格式及视频文件格式。\\n* H264: h264编码,输出mp4文件\\n* VP8:vp8编码,输出webm文件\",\n  \"type\": \"string\",\n  \"enum\": [\n    \"H264\",\n    \"VP8\",\n    \"VP9\"\n  ]\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VideoConfigCodec {
    H264,
    #[serde(rename = "VP8")]
    Vp8,
    #[serde(rename = "VP9")]
    Vp9,
}
impl From<&VideoConfigCodec> for VideoConfigCodec {
    fn from(value: &VideoConfigCodec) -> Self {
        value.clone()
    }
}
impl ToString for VideoConfigCodec {
    fn to_string(&self) -> String {
        match *self {
            Self::H264 => "H264".to_string(),
            Self::Vp8 => "VP8".to_string(),
            Self::Vp9 => "VP9".to_string(),
        }
    }
}
impl std::str::FromStr for VideoConfigCodec {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "H264" => Ok(Self::H264),
            "VP8" => Ok(Self::Vp8),
            "VP9" => Ok(Self::Vp9),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for VideoConfigCodec {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for VideoConfigCodec {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for VideoConfigCodec {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "帧率。\n\n单位:FPS。\n> *  分身数字人视频固定25FPS。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"帧率。\\n\\n单位:FPS。\\n> *  分身数字人视频固定25FPS。\",\n  \"default\": \"25\",\n  \"type\": \"string\",\n  \"format\": \"string\",\n  \"enum\": [\n    \"24\",\n    \"25\",\n    \"30\"\n  ],\n  \"maxLength\": 10,\n  \"minLength\": 0\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VideoConfigFrameRate {
    #[serde(rename = "24")]
    _24,
    #[serde(rename = "25")]
    _25,
    #[serde(rename = "30")]
    _30,
}
impl From<&VideoConfigFrameRate> for VideoConfigFrameRate {
    fn from(value: &VideoConfigFrameRate) -> Self {
        value.clone()
    }
}
impl ToString for VideoConfigFrameRate {
    fn to_string(&self) -> String {
        match *self {
            Self::_24 => "24".to_string(),
            Self::_25 => "25".to_string(),
            Self::_30 => "30".to_string(),
        }
    }
}
impl std::str::FromStr for VideoConfigFrameRate {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "24" => Ok(Self::_24),
            "25" => Ok(Self::_25),
            "30" => Ok(Self::_30),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for VideoConfigFrameRate {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for VideoConfigFrameRate {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for VideoConfigFrameRate {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for VideoConfigFrameRate {
    fn default() -> Self {
        VideoConfigFrameRate::_25
    }
}
#[doc = "素材视频图层配置。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"素材视频图层配置。\",\n  \"type\": \"object\",\n  \"properties\": {\n    \"video_cover_url\": {\n      \"description\": \"视频封面文件的URL。\",\n      \"type\": \"string\",\n      \"maxLength\": 2048,\n      \"minLength\": 1\n    },\n    \"video_url\": {\n      \"description\": \"视频文件的URL。\",\n      \"type\": \"string\",\n      \"maxLength\": 2048,\n      \"minLength\": 1\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VideoLayerConfig {
    #[doc = "视频封面文件的URL。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_cover_url: Option<VideoLayerConfigVideoCoverUrl>,
    #[doc = "视频文件的URL。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_url: Option<VideoLayerConfigVideoUrl>,
}
impl From<&VideoLayerConfig> for VideoLayerConfig {
    fn from(value: &VideoLayerConfig) -> Self {
        value.clone()
    }
}
impl VideoLayerConfig {
    pub fn builder() -> builder::VideoLayerConfig {
        Default::default()
    }
}
#[doc = "视频封面文件的URL。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"视频封面文件的URL。\",\n  \"type\": \"string\",\n  \"maxLength\": 2048,\n  \"minLength\": 1\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VideoLayerConfigVideoCoverUrl(String);
impl std::ops::Deref for VideoLayerConfigVideoCoverUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<VideoLayerConfigVideoCoverUrl> for String {
    fn from(value: VideoLayerConfigVideoCoverUrl) -> Self {
        value.0
    }
}
impl From<&VideoLayerConfigVideoCoverUrl> for VideoLayerConfigVideoCoverUrl {
    fn from(value: &VideoLayerConfigVideoCoverUrl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for VideoLayerConfigVideoCoverUrl {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 2048usize {
            return Err("longer than 2048 characters");
        }
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for VideoLayerConfigVideoCoverUrl {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for VideoLayerConfigVideoCoverUrl {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for VideoLayerConfigVideoCoverUrl {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for VideoLayerConfigVideoCoverUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "视频文件的URL。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"视频文件的URL。\",\n  \"type\": \"string\",\n  \"maxLength\": 2048,\n  \"minLength\": 1\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VideoLayerConfigVideoUrl(String);
impl std::ops::Deref for VideoLayerConfigVideoUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<VideoLayerConfigVideoUrl> for String {
    fn from(value: VideoLayerConfigVideoUrl) -> Self {
        value.0
    }
}
impl From<&VideoLayerConfigVideoUrl> for VideoLayerConfigVideoUrl {
    fn from(value: &VideoLayerConfigVideoUrl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for VideoLayerConfigVideoUrl {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 2048usize {
            return Err("longer than 2048 characters");
        }
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for VideoLayerConfigVideoUrl {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for VideoLayerConfigVideoUrl {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for VideoLayerConfigVideoUrl {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for VideoLayerConfigVideoUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "语音配置参数。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"语音配置参数。\",\n  \"required\": [\n    \"voice_asset_id\"\n  ],\n  \"properties\": {\n    \"pitch\": {\n      \"description\": \"音高。\\n\\n默认值100,最小值50,最大值200。\",\n      \"default\": 100,\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 200.0,\n      \"minimum\": 50.0\n    },\n    \"speed\": {\n      \"description\": \"语速。\\n\\n默认值100,最小值50,最大值200。\\n> * 当取值为“100”时,表示一个成年人正常的语速,约为250字/分钟。\\n> * 50表示0.5倍语速,100表示正常语速,200表示2倍语速。\",\n      \"default\": 100,\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 200.0,\n      \"minimum\": 50.0\n    },\n    \"voice_asset_id\": {\n      \"description\": \"音色资产ID。\",\n      \"type\": \"string\",\n      \"maxLength\": 64,\n      \"minLength\": 1,\n      \"example\": \"39b9cedccec1f19b33d8f7ce1a6d35b9\"\n    },\n    \"volume\": {\n      \"description\": \"音量。\\n\\n默认值140,最小值90,最大值240。\",\n      \"default\": 140,\n      \"type\": \"integer\",\n      \"format\": \"int32\",\n      \"maximum\": 240.0,\n      \"minimum\": 90.0\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceConfig {
    #[doc = "音高。\n\n默认值100,最小值50,最大值200。"]
    #[serde(default = "defaults::default_u64::<i64, 100>")]
    pub pitch: i64,
    #[doc = "语速。\n\n默认值100,最小值50,最大值200。\n> * 当取值为“100”时,表示一个成年人正常的语速,约为250字/分钟。\n> * 50表示0.5倍语速,100表示正常语速,200表示2倍语速。"]
    #[serde(default = "defaults::default_u64::<i64, 100>")]
    pub speed: i64,
    #[doc = "音色资产ID。"]
    pub voice_asset_id: VoiceConfigVoiceAssetId,
    #[doc = "音量。\n\n默认值140,最小值90,最大值240。"]
    #[serde(default = "defaults::default_u64::<i64, 140>")]
    pub volume: i64,
}
impl From<&VoiceConfig> for VoiceConfig {
    fn from(value: &VoiceConfig) -> Self {
        value.clone()
    }
}
impl VoiceConfig {
    pub fn builder() -> builder::VoiceConfig {
        Default::default()
    }
}
#[doc = "音色资产ID。"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"音色资产ID。\",\n  \"type\": \"string\",\n  \"maxLength\": 64,\n  \"minLength\": 1,\n  \"example\": \"39b9cedccec1f19b33d8f7ce1a6d35b9\"\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VoiceConfigVoiceAssetId(String);
impl std::ops::Deref for VoiceConfigVoiceAssetId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<VoiceConfigVoiceAssetId> for String {
    fn from(value: VoiceConfigVoiceAssetId) -> Self {
        value.0
    }
}
impl From<&VoiceConfigVoiceAssetId> for VoiceConfigVoiceAssetId {
    fn from(value: &VoiceConfigVoiceAssetId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for VoiceConfigVoiceAssetId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() > 64usize {
            return Err("longer than 64 characters");
        }
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for VoiceConfigVoiceAssetId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for VoiceConfigVoiceAssetId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for VoiceConfigVoiceAssetId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for VoiceConfigVoiceAssetId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BackgroundConfigInfo {
        background_asset_id: Result<Option<super::BackgroundConfigInfoBackgroundAssetId>, String>,
        background_config: Result<super::BackgroundConfigInfoBackgroundConfig, String>,
        background_cover_url: Result<Option<super::BackgroundConfigInfoBackgroundCoverUrl>, String>,
        background_title: Result<Option<super::BackgroundConfigInfoBackgroundTitle>, String>,
        background_type: Result<super::BackgroundConfigInfoBackgroundType, String>,
        human_position_2d: Result<Option<super::HumanPosition2D>, String>,
        human_size_2d: Result<Option<super::HumanSize2D>, String>,
    }
    impl Default for BackgroundConfigInfo {
        fn default() -> Self {
            Self {
                background_asset_id: Ok(Default::default()),
                background_config: Err("no value supplied for background_config".to_string()),
                background_cover_url: Ok(Default::default()),
                background_title: Ok(Default::default()),
                background_type: Err("no value supplied for background_type".to_string()),
                human_position_2d: Ok(Default::default()),
                human_size_2d: Ok(Default::default()),
            }
        }
    }
    impl BackgroundConfigInfo {
        pub fn background_asset_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BackgroundConfigInfoBackgroundAssetId>>,
            T::Error: std::fmt::Display,
        {
            self.background_asset_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for background_asset_id: {}",
                    e
                )
            });
            self
        }
        pub fn background_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::BackgroundConfigInfoBackgroundConfig>,
            T::Error: std::fmt::Display,
        {
            self.background_config = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for background_config: {}",
                    e
                )
            });
            self
        }
        pub fn background_cover_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BackgroundConfigInfoBackgroundCoverUrl>>,
            T::Error: std::fmt::Display,
        {
            self.background_cover_url = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for background_cover_url: {}",
                    e
                )
            });
            self
        }
        pub fn background_title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BackgroundConfigInfoBackgroundTitle>>,
            T::Error: std::fmt::Display,
        {
            self.background_title = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for background_title: {}",
                    e
                )
            });
            self
        }
        pub fn background_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::BackgroundConfigInfoBackgroundType>,
            T::Error: std::fmt::Display,
        {
            self.background_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for background_type: {}", e));
            self
        }
        pub fn human_position_2d<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::HumanPosition2D>>,
            T::Error: std::fmt::Display,
        {
            self.human_position_2d = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for human_position_2d: {}",
                    e
                )
            });
            self
        }
        pub fn human_size_2d<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::HumanSize2D>>,
            T::Error: std::fmt::Display,
        {
            self.human_size_2d = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for human_size_2d: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<BackgroundConfigInfo> for super::BackgroundConfigInfo {
        type Error = String;
        fn try_from(value: BackgroundConfigInfo) -> Result<Self, String> {
            Ok(Self {
                background_asset_id: value.background_asset_id?,
                background_config: value.background_config?,
                background_cover_url: value.background_cover_url?,
                background_title: value.background_title?,
                background_type: value.background_type?,
                human_position_2d: value.human_position_2d?,
                human_size_2d: value.human_size_2d?,
            })
        }
    }
    impl From<super::BackgroundConfigInfo> for BackgroundConfigInfo {
        fn from(value: super::BackgroundConfigInfo) -> Self {
            Self {
                background_asset_id: Ok(value.background_asset_id),
                background_config: Ok(value.background_config),
                background_cover_url: Ok(value.background_cover_url),
                background_title: Ok(value.background_title),
                background_type: Ok(value.background_type),
                human_position_2d: Ok(value.human_position_2d),
                human_size_2d: Ok(value.human_size_2d),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateSmartLiveRoomReq {
        backup_model_asset_ids:
            Result<Vec<super::CreateSmartLiveRoomReqBackupModelAssetIdsItem>, String>,
        interaction_rules: Result<Vec<super::InteractionRuleInfo>, String>,
        live_event_callback_config: Result<Option<super::LiveEventCallBackConfig>, String>,
        output_urls: Result<Vec<super::CreateSmartLiveRoomReqOutputUrlsItem>, String>,
        play_policy: Result<Option<super::PlayPolicy>, String>,
        review_config: Result<Option<super::ReviewConfig>, String>,
        room_description: Result<Option<super::CreateSmartLiveRoomReqRoomDescription>, String>,
        room_name: Result<super::CreateSmartLiveRoomReqRoomName, String>,
        room_type: Result<super::CreateSmartLiveRoomReqRoomType, String>,
        scene_scripts: Result<Vec<super::LiveVideoScriptInfo>, String>,
        shared_config: Result<Option<super::SharedConfig>, String>,
        stream_keys: Result<Vec<super::CreateSmartLiveRoomReqStreamKeysItem>, String>,
        video_config: Result<Option<super::VideoConfig>, String>,
    }
    impl Default for CreateSmartLiveRoomReq {
        fn default() -> Self {
            Self {
                backup_model_asset_ids: Ok(Default::default()),
                interaction_rules: Ok(Default::default()),
                live_event_callback_config: Ok(Default::default()),
                output_urls: Ok(Default::default()),
                play_policy: Ok(Default::default()),
                review_config: Ok(Default::default()),
                room_description: Ok(Default::default()),
                room_name: Err("no value supplied for room_name".to_string()),
                room_type: Ok(super::defaults::create_smart_live_room_req_room_type()),
                scene_scripts: Ok(Default::default()),
                shared_config: Ok(Default::default()),
                stream_keys: Ok(Default::default()),
                video_config: Ok(Default::default()),
            }
        }
    }
    impl CreateSmartLiveRoomReq {
        pub fn backup_model_asset_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::CreateSmartLiveRoomReqBackupModelAssetIdsItem>>,
            T::Error: std::fmt::Display,
        {
            self.backup_model_asset_ids = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for backup_model_asset_ids: {}",
                    e
                )
            });
            self
        }
        pub fn interaction_rules<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::InteractionRuleInfo>>,
            T::Error: std::fmt::Display,
        {
            self.interaction_rules = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for interaction_rules: {}",
                    e
                )
            });
            self
        }
        pub fn live_event_callback_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LiveEventCallBackConfig>>,
            T::Error: std::fmt::Display,
        {
            self.live_event_callback_config = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for live_event_callback_config: {}",
                    e
                )
            });
            self
        }
        pub fn output_urls<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::CreateSmartLiveRoomReqOutputUrlsItem>>,
            T::Error: std::fmt::Display,
        {
            self.output_urls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_urls: {}", e));
            self
        }
        pub fn play_policy<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::PlayPolicy>>,
            T::Error: std::fmt::Display,
        {
            self.play_policy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for play_policy: {}", e));
            self
        }
        pub fn review_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ReviewConfig>>,
            T::Error: std::fmt::Display,
        {
            self.review_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for review_config: {}", e));
            self
        }
        pub fn room_description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CreateSmartLiveRoomReqRoomDescription>>,
            T::Error: std::fmt::Display,
        {
            self.room_description = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for room_description: {}",
                    e
                )
            });
            self
        }
        pub fn room_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CreateSmartLiveRoomReqRoomName>,
            T::Error: std::fmt::Display,
        {
            self.room_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for room_name: {}", e));
            self
        }
        pub fn room_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::CreateSmartLiveRoomReqRoomType>,
            T::Error: std::fmt::Display,
        {
            self.room_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for room_type: {}", e));
            self
        }
        pub fn scene_scripts<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::LiveVideoScriptInfo>>,
            T::Error: std::fmt::Display,
        {
            self.scene_scripts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scene_scripts: {}", e));
            self
        }
        pub fn shared_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SharedConfig>>,
            T::Error: std::fmt::Display,
        {
            self.shared_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shared_config: {}", e));
            self
        }
        pub fn stream_keys<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::CreateSmartLiveRoomReqStreamKeysItem>>,
            T::Error: std::fmt::Display,
        {
            self.stream_keys = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stream_keys: {}", e));
            self
        }
        pub fn video_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::VideoConfig>>,
            T::Error: std::fmt::Display,
        {
            self.video_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_config: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CreateSmartLiveRoomReq> for super::CreateSmartLiveRoomReq {
        type Error = String;
        fn try_from(value: CreateSmartLiveRoomReq) -> Result<Self, String> {
            Ok(Self {
                backup_model_asset_ids: value.backup_model_asset_ids?,
                interaction_rules: value.interaction_rules?,
                live_event_callback_config: value.live_event_callback_config?,
                output_urls: value.output_urls?,
                play_policy: value.play_policy?,
                review_config: value.review_config?,
                room_description: value.room_description?,
                room_name: value.room_name?,
                room_type: value.room_type?,
                scene_scripts: value.scene_scripts?,
                shared_config: value.shared_config?,
                stream_keys: value.stream_keys?,
                video_config: value.video_config?,
            })
        }
    }
    impl From<super::CreateSmartLiveRoomReq> for CreateSmartLiveRoomReq {
        fn from(value: super::CreateSmartLiveRoomReq) -> Self {
            Self {
                backup_model_asset_ids: Ok(value.backup_model_asset_ids),
                interaction_rules: Ok(value.interaction_rules),
                live_event_callback_config: Ok(value.live_event_callback_config),
                output_urls: Ok(value.output_urls),
                play_policy: Ok(value.play_policy),
                review_config: Ok(value.review_config),
                room_description: Ok(value.room_description),
                room_name: Ok(value.room_name),
                room_type: Ok(value.room_type),
                scene_scripts: Ok(value.scene_scripts),
                shared_config: Ok(value.shared_config),
                stream_keys: Ok(value.stream_keys),
                video_config: Ok(value.video_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CreateSmartLiveRoomRsp {
        room_id: Result<Option<super::CreateSmartLiveRoomRspRoomId>, String>,
    }
    impl Default for CreateSmartLiveRoomRsp {
        fn default() -> Self {
            Self {
                room_id: Ok(Default::default()),
            }
        }
    }
    impl CreateSmartLiveRoomRsp {
        pub fn room_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::CreateSmartLiveRoomRspRoomId>>,
            T::Error: std::fmt::Display,
        {
            self.room_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for room_id: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CreateSmartLiveRoomRsp> for super::CreateSmartLiveRoomRsp {
        type Error = String;
        fn try_from(value: CreateSmartLiveRoomRsp) -> Result<Self, String> {
            Ok(Self {
                room_id: value.room_id?,
            })
        }
    }
    impl From<super::CreateSmartLiveRoomRsp> for CreateSmartLiveRoomRsp {
        fn from(value: super::CreateSmartLiveRoomRsp) -> Self {
            Self {
                room_id: Ok(value.room_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HitCondition {
        priority: Result<Option<i64>, String>,
        relation: Result<Option<super::HitConditionRelation>, String>,
        tags: Result<Vec<super::HitConditionTag>, String>,
    }
    impl Default for HitCondition {
        fn default() -> Self {
            Self {
                priority: Ok(Default::default()),
                relation: Ok(Default::default()),
                tags: Ok(Default::default()),
            }
        }
    }
    impl HitCondition {
        pub fn priority<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.priority = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for priority: {}", e));
            self
        }
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::HitConditionRelation>>,
            T::Error: std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for relation: {}", e));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::HitConditionTag>>,
            T::Error: std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<HitCondition> for super::HitCondition {
        type Error = String;
        fn try_from(value: HitCondition) -> Result<Self, String> {
            Ok(Self {
                priority: value.priority?,
                relation: value.relation?,
                tags: value.tags?,
            })
        }
    }
    impl From<super::HitCondition> for HitCondition {
        fn from(value: super::HitCondition) -> Self {
            Self {
                priority: Ok(value.priority),
                relation: Ok(value.relation),
                tags: Ok(value.tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HitConditionTag {
        match_: Result<Option<super::HitConditionTagMatch>, String>,
        operation: Result<super::HitConditionTagOperation, String>,
        tag: Result<Option<super::HitConditionTagTag>, String>,
        value: Result<Option<super::HitConditionTagValue>, String>,
    }
    impl Default for HitConditionTag {
        fn default() -> Self {
            Self {
                match_: Ok(Default::default()),
                operation: Ok(super::defaults::hit_condition_tag_operation()),
                tag: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl HitConditionTag {
        pub fn match_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::HitConditionTagMatch>>,
            T::Error: std::fmt::Display,
        {
            self.match_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for match_: {}", e));
            self
        }
        pub fn operation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::HitConditionTagOperation>,
            T::Error: std::fmt::Display,
        {
            self.operation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operation: {}", e));
            self
        }
        pub fn tag<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::HitConditionTagTag>>,
            T::Error: std::fmt::Display,
        {
            self.tag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tag: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::HitConditionTagValue>>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<HitConditionTag> for super::HitConditionTag {
        type Error = String;
        fn try_from(value: HitConditionTag) -> Result<Self, String> {
            Ok(Self {
                match_: value.match_?,
                operation: value.operation?,
                tag: value.tag?,
                value: value.value?,
            })
        }
    }
    impl From<super::HitConditionTag> for HitConditionTag {
        fn from(value: super::HitConditionTag) -> Self {
            Self {
                match_: Ok(value.match_),
                operation: Ok(value.operation),
                tag: Ok(value.tag),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HumanPosition2D {
        position: Result<super::HumanPosition2DPosition, String>,
        position_x: Result<Option<i64>, String>,
        position_y: Result<Option<i64>, String>,
    }
    impl Default for HumanPosition2D {
        fn default() -> Self {
            Self {
                position: Ok(super::defaults::human_position2_d_position()),
                position_x: Ok(Default::default()),
                position_y: Ok(Default::default()),
            }
        }
    }
    impl HumanPosition2D {
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::HumanPosition2DPosition>,
            T::Error: std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {}", e));
            self
        }
        pub fn position_x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.position_x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position_x: {}", e));
            self
        }
        pub fn position_y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.position_y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position_y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<HumanPosition2D> for super::HumanPosition2D {
        type Error = String;
        fn try_from(value: HumanPosition2D) -> Result<Self, String> {
            Ok(Self {
                position: value.position?,
                position_x: value.position_x?,
                position_y: value.position_y?,
            })
        }
    }
    impl From<super::HumanPosition2D> for HumanPosition2D {
        fn from(value: super::HumanPosition2D) -> Self {
            Self {
                position: Ok(value.position),
                position_x: Ok(value.position_x),
                position_y: Ok(value.position_y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HumanSize2D {
        height: Result<Option<i64>, String>,
        width: Result<Option<i64>, String>,
    }
    impl Default for HumanSize2D {
        fn default() -> Self {
            Self {
                height: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl HumanSize2D {
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height: {}", e));
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for width: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<HumanSize2D> for super::HumanSize2D {
        type Error = String;
        fn try_from(value: HumanSize2D) -> Result<Self, String> {
            Ok(Self {
                height: value.height?,
                width: value.width?,
            })
        }
    }
    impl From<super::HumanSize2D> for HumanSize2D {
        fn from(value: super::HumanSize2D) -> Self {
            Self {
                height: Ok(value.height),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ImageLayerConfig {
        image_url: Result<Option<super::ImageLayerConfigImageUrl>, String>,
    }
    impl Default for ImageLayerConfig {
        fn default() -> Self {
            Self {
                image_url: Ok(Default::default()),
            }
        }
    }
    impl ImageLayerConfig {
        pub fn image_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ImageLayerConfigImageUrl>>,
            T::Error: std::fmt::Display,
        {
            self.image_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ImageLayerConfig> for super::ImageLayerConfig {
        type Error = String;
        fn try_from(value: ImageLayerConfig) -> Result<Self, String> {
            Ok(Self {
                image_url: value.image_url?,
            })
        }
    }
    impl From<super::ImageLayerConfig> for ImageLayerConfig {
        fn from(value: super::ImageLayerConfig) -> Self {
            Self {
                image_url: Ok(value.image_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InteractionRuleInfo {
        enabled: Result<Option<bool>, String>,
        event_type: Result<Option<i64>, String>,
        hit_condition: Result<Option<super::HitCondition>, String>,
        review_config: Result<Option<super::ReviewConfig>, String>,
        rule_index: Result<Option<super::InteractionRuleInfoRuleIndex>, String>,
        rule_name: Result<Option<super::InteractionRuleInfoRuleName>, String>,
        trigger: Result<Option<super::TriggerProcess>, String>,
    }
    impl Default for InteractionRuleInfo {
        fn default() -> Self {
            Self {
                enabled: Ok(Default::default()),
                event_type: Ok(Default::default()),
                hit_condition: Ok(Default::default()),
                review_config: Ok(Default::default()),
                rule_index: Ok(Default::default()),
                rule_name: Ok(Default::default()),
                trigger: Ok(Default::default()),
            }
        }
    }
    impl InteractionRuleInfo {
        pub fn enabled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.enabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enabled: {}", e));
            self
        }
        pub fn event_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.event_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_type: {}", e));
            self
        }
        pub fn hit_condition<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::HitCondition>>,
            T::Error: std::fmt::Display,
        {
            self.hit_condition = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hit_condition: {}", e));
            self
        }
        pub fn review_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ReviewConfig>>,
            T::Error: std::fmt::Display,
        {
            self.review_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for review_config: {}", e));
            self
        }
        pub fn rule_index<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::InteractionRuleInfoRuleIndex>>,
            T::Error: std::fmt::Display,
        {
            self.rule_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rule_index: {}", e));
            self
        }
        pub fn rule_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::InteractionRuleInfoRuleName>>,
            T::Error: std::fmt::Display,
        {
            self.rule_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rule_name: {}", e));
            self
        }
        pub fn trigger<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TriggerProcess>>,
            T::Error: std::fmt::Display,
        {
            self.trigger = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trigger: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<InteractionRuleInfo> for super::InteractionRuleInfo {
        type Error = String;
        fn try_from(value: InteractionRuleInfo) -> Result<Self, String> {
            Ok(Self {
                enabled: value.enabled?,
                event_type: value.event_type?,
                hit_condition: value.hit_condition?,
                review_config: value.review_config?,
                rule_index: value.rule_index?,
                rule_name: value.rule_name?,
                trigger: value.trigger?,
            })
        }
    }
    impl From<super::InteractionRuleInfo> for InteractionRuleInfo {
        fn from(value: super::InteractionRuleInfo) -> Self {
            Self {
                enabled: Ok(value.enabled),
                event_type: Ok(value.event_type),
                hit_condition: Ok(value.hit_condition),
                review_config: Ok(value.review_config),
                rule_index: Ok(value.rule_index),
                rule_name: Ok(value.rule_name),
                trigger: Ok(value.trigger),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LayerConfig {
        group_id: Result<Option<super::LayerConfigGroupId>, String>,
        image_config: Result<Option<super::ImageLayerConfig>, String>,
        layer_type: Result<super::LayerConfigLayerType, String>,
        position: Result<super::LayerPositionConfig, String>,
        size: Result<Option<super::LayerSizeConfig>, String>,
        text_config: Result<Option<super::TextLayerConfig>, String>,
        video_config: Result<Option<super::VideoLayerConfig>, String>,
    }
    impl Default for LayerConfig {
        fn default() -> Self {
            Self {
                group_id: Ok(Default::default()),
                image_config: Ok(Default::default()),
                layer_type: Err("no value supplied for layer_type".to_string()),
                position: Err("no value supplied for position".to_string()),
                size: Ok(Default::default()),
                text_config: Ok(Default::default()),
                video_config: Ok(Default::default()),
            }
        }
    }
    impl LayerConfig {
        pub fn group_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LayerConfigGroupId>>,
            T::Error: std::fmt::Display,
        {
            self.group_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_id: {}", e));
            self
        }
        pub fn image_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ImageLayerConfig>>,
            T::Error: std::fmt::Display,
        {
            self.image_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_config: {}", e));
            self
        }
        pub fn layer_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LayerConfigLayerType>,
            T::Error: std::fmt::Display,
        {
            self.layer_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layer_type: {}", e));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LayerPositionConfig>,
            T::Error: std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {}", e));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LayerSizeConfig>>,
            T::Error: std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn text_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TextLayerConfig>>,
            T::Error: std::fmt::Display,
        {
            self.text_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_config: {}", e));
            self
        }
        pub fn video_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::VideoLayerConfig>>,
            T::Error: std::fmt::Display,
        {
            self.video_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_config: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LayerConfig> for super::LayerConfig {
        type Error = String;
        fn try_from(value: LayerConfig) -> Result<Self, String> {
            Ok(Self {
                group_id: value.group_id?,
                image_config: value.image_config?,
                layer_type: value.layer_type?,
                position: value.position?,
                size: value.size?,
                text_config: value.text_config?,
                video_config: value.video_config?,
            })
        }
    }
    impl From<super::LayerConfig> for LayerConfig {
        fn from(value: super::LayerConfig) -> Self {
            Self {
                group_id: Ok(value.group_id),
                image_config: Ok(value.image_config),
                layer_type: Ok(value.layer_type),
                position: Ok(value.position),
                size: Ok(value.size),
                text_config: Ok(value.text_config),
                video_config: Ok(value.video_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LayerPositionConfig {
        dx: Result<i64, String>,
        dy: Result<i64, String>,
        layer_index: Result<i64, String>,
    }
    impl Default for LayerPositionConfig {
        fn default() -> Self {
            Self {
                dx: Err("no value supplied for dx".to_string()),
                dy: Err("no value supplied for dy".to_string()),
                layer_index: Err("no value supplied for layer_index".to_string()),
            }
        }
    }
    impl LayerPositionConfig {
        pub fn dx<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.dx = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dx: {}", e));
            self
        }
        pub fn dy<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.dy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dy: {}", e));
            self
        }
        pub fn layer_index<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.layer_index = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layer_index: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LayerPositionConfig> for super::LayerPositionConfig {
        type Error = String;
        fn try_from(value: LayerPositionConfig) -> Result<Self, String> {
            Ok(Self {
                dx: value.dx?,
                dy: value.dy?,
                layer_index: value.layer_index?,
            })
        }
    }
    impl From<super::LayerPositionConfig> for LayerPositionConfig {
        fn from(value: super::LayerPositionConfig) -> Self {
            Self {
                dx: Ok(value.dx),
                dy: Ok(value.dy),
                layer_index: Ok(value.layer_index),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LayerSizeConfig {
        height: Result<Option<i64>, String>,
        width: Result<Option<i64>, String>,
    }
    impl Default for LayerSizeConfig {
        fn default() -> Self {
            Self {
                height: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl LayerSizeConfig {
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height: {}", e));
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for width: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LayerSizeConfig> for super::LayerSizeConfig {
        type Error = String;
        fn try_from(value: LayerSizeConfig) -> Result<Self, String> {
            Ok(Self {
                height: value.height?,
                width: value.width?,
            })
        }
    }
    impl From<super::LayerSizeConfig> for LayerSizeConfig {
        fn from(value: super::LayerSizeConfig) -> Self {
            Self {
                height: Ok(value.height),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveAudioConfig {
        audio_url: Result<Option<super::LiveAudioConfigAudioUrl>, String>,
        subtitle_url: Result<Option<super::LiveAudioConfigSubtitleUrl>, String>,
    }
    impl Default for LiveAudioConfig {
        fn default() -> Self {
            Self {
                audio_url: Ok(Default::default()),
                subtitle_url: Ok(Default::default()),
            }
        }
    }
    impl LiveAudioConfig {
        pub fn audio_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LiveAudioConfigAudioUrl>>,
            T::Error: std::fmt::Display,
        {
            self.audio_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio_url: {}", e));
            self
        }
        pub fn subtitle_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LiveAudioConfigSubtitleUrl>>,
            T::Error: std::fmt::Display,
        {
            self.subtitle_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtitle_url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LiveAudioConfig> for super::LiveAudioConfig {
        type Error = String;
        fn try_from(value: LiveAudioConfig) -> Result<Self, String> {
            Ok(Self {
                audio_url: value.audio_url?,
                subtitle_url: value.subtitle_url?,
            })
        }
    }
    impl From<super::LiveAudioConfig> for LiveAudioConfig {
        fn from(value: super::LiveAudioConfig) -> Self {
            Self {
                audio_url: Ok(value.audio_url),
                subtitle_url: Ok(value.subtitle_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveEventCallBackConfig {
        auth_type: Result<super::LiveEventCallBackConfigAuthType, String>,
        callback_event_type:
            Result<Vec<super::LiveEventCallBackConfigCallbackEventTypeItem>, String>,
        key: Result<Option<super::LiveEventCallBackConfigKey>, String>,
        live_event_type_callback_url:
            Result<Option<super::LiveEventCallBackConfigLiveEventTypeCallbackUrl>, String>,
    }
    impl Default for LiveEventCallBackConfig {
        fn default() -> Self {
            Self {
                auth_type: Ok(super::defaults::live_event_call_back_config_auth_type()),
                callback_event_type: Ok(Default::default()),
                key: Ok(Default::default()),
                live_event_type_callback_url: Ok(Default::default()),
            }
        }
    }
    impl LiveEventCallBackConfig {
        pub fn auth_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LiveEventCallBackConfigAuthType>,
            T::Error: std::fmt::Display,
        {
            self.auth_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for auth_type: {}", e));
            self
        }
        pub fn callback_event_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::LiveEventCallBackConfigCallbackEventTypeItem>>,
            T::Error: std::fmt::Display,
        {
            self.callback_event_type = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for callback_event_type: {}",
                    e
                )
            });
            self
        }
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LiveEventCallBackConfigKey>>,
            T::Error: std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {}", e));
            self
        }
        pub fn live_event_type_callback_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                Option<super::LiveEventCallBackConfigLiveEventTypeCallbackUrl>,
            >,
            T::Error: std::fmt::Display,
        {
            self.live_event_type_callback_url = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for live_event_type_callback_url: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<LiveEventCallBackConfig> for super::LiveEventCallBackConfig {
        type Error = String;
        fn try_from(value: LiveEventCallBackConfig) -> Result<Self, String> {
            Ok(Self {
                auth_type: value.auth_type?,
                callback_event_type: value.callback_event_type?,
                key: value.key?,
                live_event_type_callback_url: value.live_event_type_callback_url?,
            })
        }
    }
    impl From<super::LiveEventCallBackConfig> for LiveEventCallBackConfig {
        fn from(value: super::LiveEventCallBackConfig) -> Self {
            Self {
                auth_type: Ok(value.auth_type),
                callback_event_type: Ok(value.callback_event_type),
                key: Ok(value.key),
                live_event_type_callback_url: Ok(value.live_event_type_callback_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveShootScriptItem {
        audio_config: Result<Option<super::LiveAudioConfig>, String>,
        sequence_no: Result<Option<i64>, String>,
        text_config: Result<Option<super::TextConfig>, String>,
        title: Result<Option<super::LiveShootScriptItemTitle>, String>,
    }
    impl Default for LiveShootScriptItem {
        fn default() -> Self {
            Self {
                audio_config: Ok(Default::default()),
                sequence_no: Ok(Default::default()),
                text_config: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl LiveShootScriptItem {
        pub fn audio_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LiveAudioConfig>>,
            T::Error: std::fmt::Display,
        {
            self.audio_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio_config: {}", e));
            self
        }
        pub fn sequence_no<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.sequence_no = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sequence_no: {}", e));
            self
        }
        pub fn text_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TextConfig>>,
            T::Error: std::fmt::Display,
        {
            self.text_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_config: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LiveShootScriptItemTitle>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LiveShootScriptItem> for super::LiveShootScriptItem {
        type Error = String;
        fn try_from(value: LiveShootScriptItem) -> Result<Self, String> {
            Ok(Self {
                audio_config: value.audio_config?,
                sequence_no: value.sequence_no?,
                text_config: value.text_config?,
                title: value.title?,
            })
        }
    }
    impl From<super::LiveShootScriptItem> for LiveShootScriptItem {
        fn from(value: super::LiveShootScriptItem) -> Self {
            Self {
                audio_config: Ok(value.audio_config),
                sequence_no: Ok(value.sequence_no),
                text_config: Ok(value.text_config),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiveVideoScriptInfo {
        background_config: Result<Vec<super::BackgroundConfigInfo>, String>,
        dh_id: Result<Option<super::LiveVideoScriptInfoDhId>, String>,
        layer_config: Result<Vec<super::LayerConfig>, String>,
        model_asset_id: Result<Option<super::LiveVideoScriptInfoModelAssetId>, String>,
        script_description: Result<Option<super::LiveVideoScriptInfoScriptDescription>, String>,
        script_name: Result<super::LiveVideoScriptInfoScriptName, String>,
        shoot_scripts: Result<Vec<super::LiveShootScriptItem>, String>,
        voice_config: Result<Option<super::VoiceConfig>, String>,
    }
    impl Default for LiveVideoScriptInfo {
        fn default() -> Self {
            Self {
                background_config: Ok(Default::default()),
                dh_id: Ok(Default::default()),
                layer_config: Ok(Default::default()),
                model_asset_id: Ok(Default::default()),
                script_description: Ok(Default::default()),
                script_name: Err("no value supplied for script_name".to_string()),
                shoot_scripts: Err("no value supplied for shoot_scripts".to_string()),
                voice_config: Ok(Default::default()),
            }
        }
    }
    impl LiveVideoScriptInfo {
        pub fn background_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::BackgroundConfigInfo>>,
            T::Error: std::fmt::Display,
        {
            self.background_config = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for background_config: {}",
                    e
                )
            });
            self
        }
        pub fn dh_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LiveVideoScriptInfoDhId>>,
            T::Error: std::fmt::Display,
        {
            self.dh_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dh_id: {}", e));
            self
        }
        pub fn layer_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::LayerConfig>>,
            T::Error: std::fmt::Display,
        {
            self.layer_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layer_config: {}", e));
            self
        }
        pub fn model_asset_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LiveVideoScriptInfoModelAssetId>>,
            T::Error: std::fmt::Display,
        {
            self.model_asset_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for model_asset_id: {}", e));
            self
        }
        pub fn script_description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LiveVideoScriptInfoScriptDescription>>,
            T::Error: std::fmt::Display,
        {
            self.script_description = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for script_description: {}",
                    e
                )
            });
            self
        }
        pub fn script_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LiveVideoScriptInfoScriptName>,
            T::Error: std::fmt::Display,
        {
            self.script_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for script_name: {}", e));
            self
        }
        pub fn shoot_scripts<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::LiveShootScriptItem>>,
            T::Error: std::fmt::Display,
        {
            self.shoot_scripts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shoot_scripts: {}", e));
            self
        }
        pub fn voice_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::VoiceConfig>>,
            T::Error: std::fmt::Display,
        {
            self.voice_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice_config: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LiveVideoScriptInfo> for super::LiveVideoScriptInfo {
        type Error = String;
        fn try_from(value: LiveVideoScriptInfo) -> Result<Self, String> {
            Ok(Self {
                background_config: value.background_config?,
                dh_id: value.dh_id?,
                layer_config: value.layer_config?,
                model_asset_id: value.model_asset_id?,
                script_description: value.script_description?,
                script_name: value.script_name?,
                shoot_scripts: value.shoot_scripts?,
                voice_config: value.voice_config?,
            })
        }
    }
    impl From<super::LiveVideoScriptInfo> for LiveVideoScriptInfo {
        fn from(value: super::LiveVideoScriptInfo) -> Self {
            Self {
                background_config: Ok(value.background_config),
                dh_id: Ok(value.dh_id),
                layer_config: Ok(value.layer_config),
                model_asset_id: Ok(value.model_asset_id),
                script_description: Ok(value.script_description),
                script_name: Ok(value.script_name),
                shoot_scripts: Ok(value.shoot_scripts),
                voice_config: Ok(value.voice_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PlayPolicy {
        auto_play_script: Result<bool, String>,
        play_mode: Result<super::PlayPolicyPlayMode, String>,
        random_play_mode: Result<super::PlayPolicyRandomPlayMode, String>,
        repeat_count: Result<i64, String>,
    }
    impl Default for PlayPolicy {
        fn default() -> Self {
            Self {
                auto_play_script: Ok(super::defaults::default_bool::<true>()),
                play_mode: Ok(super::defaults::play_policy_play_mode()),
                random_play_mode: Ok(super::defaults::play_policy_random_play_mode()),
                repeat_count: Ok(Default::default()),
            }
        }
    }
    impl PlayPolicy {
        pub fn auto_play_script<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.auto_play_script = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for auto_play_script: {}",
                    e
                )
            });
            self
        }
        pub fn play_mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PlayPolicyPlayMode>,
            T::Error: std::fmt::Display,
        {
            self.play_mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for play_mode: {}", e));
            self
        }
        pub fn random_play_mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PlayPolicyRandomPlayMode>,
            T::Error: std::fmt::Display,
        {
            self.random_play_mode = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for random_play_mode: {}",
                    e
                )
            });
            self
        }
        pub fn repeat_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.repeat_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repeat_count: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<PlayPolicy> for super::PlayPolicy {
        type Error = String;
        fn try_from(value: PlayPolicy) -> Result<Self, String> {
            Ok(Self {
                auto_play_script: value.auto_play_script?,
                play_mode: value.play_mode?,
                random_play_mode: value.random_play_mode?,
                repeat_count: value.repeat_count?,
            })
        }
    }
    impl From<super::PlayPolicy> for PlayPolicy {
        fn from(value: super::PlayPolicy) -> Self {
            Self {
                auto_play_script: Ok(value.auto_play_script),
                play_mode: Ok(value.play_mode),
                random_play_mode: Ok(value.random_play_mode),
                repeat_count: Ok(value.repeat_count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReplyAudioInfo {
        audio_name: Result<Option<super::ReplyAudioInfoAudioName>, String>,
        audio_url: Result<Option<super::ReplyAudioInfoAudioUrl>, String>,
    }
    impl Default for ReplyAudioInfo {
        fn default() -> Self {
            Self {
                audio_name: Ok(Default::default()),
                audio_url: Ok(Default::default()),
            }
        }
    }
    impl ReplyAudioInfo {
        pub fn audio_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ReplyAudioInfoAudioName>>,
            T::Error: std::fmt::Display,
        {
            self.audio_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio_name: {}", e));
            self
        }
        pub fn audio_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ReplyAudioInfoAudioUrl>>,
            T::Error: std::fmt::Display,
        {
            self.audio_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for audio_url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ReplyAudioInfo> for super::ReplyAudioInfo {
        type Error = String;
        fn try_from(value: ReplyAudioInfo) -> Result<Self, String> {
            Ok(Self {
                audio_name: value.audio_name?,
                audio_url: value.audio_url?,
            })
        }
    }
    impl From<super::ReplyAudioInfo> for ReplyAudioInfo {
        fn from(value: super::ReplyAudioInfo) -> Self {
            Self {
                audio_name: Ok(value.audio_name),
                audio_url: Ok(value.audio_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReviewConfig {
        no_need_review: Result<Option<bool>, String>,
    }
    impl Default for ReviewConfig {
        fn default() -> Self {
            Self {
                no_need_review: Ok(Default::default()),
            }
        }
    }
    impl ReviewConfig {
        pub fn no_need_review<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.no_need_review = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for no_need_review: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ReviewConfig> for super::ReviewConfig {
        type Error = String;
        fn try_from(value: ReviewConfig) -> Result<Self, String> {
            Ok(Self {
                no_need_review: value.no_need_review?,
            })
        }
    }
    impl From<super::ReviewConfig> for ReviewConfig {
        fn from(value: super::ReviewConfig) -> Self {
            Self {
                no_need_review: Ok(value.no_need_review),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SharedConfig {
        allowed_project_ids: Result<Vec<super::SharedConfigAllowedProjectIdsItem>, String>,
        expire_time: Result<Option<super::SharedConfigExpireTime>, String>,
        shared_state: Result<Option<super::SharedConfigSharedState>, String>,
        shared_type: Result<Option<super::SharedConfigSharedType>, String>,
    }
    impl Default for SharedConfig {
        fn default() -> Self {
            Self {
                allowed_project_ids: Ok(Default::default()),
                expire_time: Ok(Default::default()),
                shared_state: Ok(Default::default()),
                shared_type: Ok(Default::default()),
            }
        }
    }
    impl SharedConfig {
        pub fn allowed_project_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::SharedConfigAllowedProjectIdsItem>>,
            T::Error: std::fmt::Display,
        {
            self.allowed_project_ids = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for allowed_project_ids: {}",
                    e
                )
            });
            self
        }
        pub fn expire_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SharedConfigExpireTime>>,
            T::Error: std::fmt::Display,
        {
            self.expire_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expire_time: {}", e));
            self
        }
        pub fn shared_state<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SharedConfigSharedState>>,
            T::Error: std::fmt::Display,
        {
            self.shared_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shared_state: {}", e));
            self
        }
        pub fn shared_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SharedConfigSharedType>>,
            T::Error: std::fmt::Display,
        {
            self.shared_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shared_type: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SharedConfig> for super::SharedConfig {
        type Error = String;
        fn try_from(value: SharedConfig) -> Result<Self, String> {
            Ok(Self {
                allowed_project_ids: value.allowed_project_ids?,
                expire_time: value.expire_time?,
                shared_state: value.shared_state?,
                shared_type: value.shared_type?,
            })
        }
    }
    impl From<super::SharedConfig> for SharedConfig {
        fn from(value: super::SharedConfig) -> Self {
            Self {
                allowed_project_ids: Ok(value.allowed_project_ids),
                expire_time: Ok(value.expire_time),
                shared_state: Ok(value.shared_state),
                shared_type: Ok(value.shared_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SmartImageLayerConfig {
        display_duration: Result<Option<i64>, String>,
        image_url: Result<super::SmartImageLayerConfigImageUrl, String>,
    }
    impl Default for SmartImageLayerConfig {
        fn default() -> Self {
            Self {
                display_duration: Ok(Default::default()),
                image_url: Err("no value supplied for image_url".to_string()),
            }
        }
    }
    impl SmartImageLayerConfig {
        pub fn display_duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.display_duration = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for display_duration: {}",
                    e
                )
            });
            self
        }
        pub fn image_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SmartImageLayerConfigImageUrl>,
            T::Error: std::fmt::Display,
        {
            self.image_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SmartImageLayerConfig> for super::SmartImageLayerConfig {
        type Error = String;
        fn try_from(value: SmartImageLayerConfig) -> Result<Self, String> {
            Ok(Self {
                display_duration: value.display_duration?,
                image_url: value.image_url?,
            })
        }
    }
    impl From<super::SmartImageLayerConfig> for SmartImageLayerConfig {
        fn from(value: super::SmartImageLayerConfig) -> Self {
            Self {
                display_duration: Ok(value.display_duration),
                image_url: Ok(value.image_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SmartLayerConfig {
        image_config: Result<Option<super::SmartImageLayerConfig>, String>,
        layer_type: Result<super::SmartLayerConfigLayerType, String>,
        position: Result<super::LayerPositionConfig, String>,
        size: Result<Option<super::LayerSizeConfig>, String>,
        video_config: Result<Option<super::SmartVideoLayerConfig>, String>,
    }
    impl Default for SmartLayerConfig {
        fn default() -> Self {
            Self {
                image_config: Ok(Default::default()),
                layer_type: Err("no value supplied for layer_type".to_string()),
                position: Err("no value supplied for position".to_string()),
                size: Ok(Default::default()),
                video_config: Ok(Default::default()),
            }
        }
    }
    impl SmartLayerConfig {
        pub fn image_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SmartImageLayerConfig>>,
            T::Error: std::fmt::Display,
        {
            self.image_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_config: {}", e));
            self
        }
        pub fn layer_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SmartLayerConfigLayerType>,
            T::Error: std::fmt::Display,
        {
            self.layer_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layer_type: {}", e));
            self
        }
        pub fn position<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LayerPositionConfig>,
            T::Error: std::fmt::Display,
        {
            self.position = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for position: {}", e));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LayerSizeConfig>>,
            T::Error: std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn video_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SmartVideoLayerConfig>>,
            T::Error: std::fmt::Display,
        {
            self.video_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_config: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SmartLayerConfig> for super::SmartLayerConfig {
        type Error = String;
        fn try_from(value: SmartLayerConfig) -> Result<Self, String> {
            Ok(Self {
                image_config: value.image_config?,
                layer_type: value.layer_type?,
                position: value.position?,
                size: value.size?,
                video_config: value.video_config?,
            })
        }
    }
    impl From<super::SmartLayerConfig> for SmartLayerConfig {
        fn from(value: super::SmartLayerConfig) -> Self {
            Self {
                image_config: Ok(value.image_config),
                layer_type: Ok(value.layer_type),
                position: Ok(value.position),
                size: Ok(value.size),
                video_config: Ok(value.video_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SmartVideoLayerConfig {
        display_duration: Result<Option<i64>, String>,
        video_cover_url: Result<Option<super::SmartVideoLayerConfigVideoCoverUrl>, String>,
        video_url: Result<super::SmartVideoLayerConfigVideoUrl, String>,
    }
    impl Default for SmartVideoLayerConfig {
        fn default() -> Self {
            Self {
                display_duration: Ok(Default::default()),
                video_cover_url: Ok(Default::default()),
                video_url: Err("no value supplied for video_url".to_string()),
            }
        }
    }
    impl SmartVideoLayerConfig {
        pub fn display_duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.display_duration = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for display_duration: {}",
                    e
                )
            });
            self
        }
        pub fn video_cover_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SmartVideoLayerConfigVideoCoverUrl>>,
            T::Error: std::fmt::Display,
        {
            self.video_cover_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_cover_url: {}", e));
            self
        }
        pub fn video_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SmartVideoLayerConfigVideoUrl>,
            T::Error: std::fmt::Display,
        {
            self.video_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SmartVideoLayerConfig> for super::SmartVideoLayerConfig {
        type Error = String;
        fn try_from(value: SmartVideoLayerConfig) -> Result<Self, String> {
            Ok(Self {
                display_duration: value.display_duration?,
                video_cover_url: value.video_cover_url?,
                video_url: value.video_url?,
            })
        }
    }
    impl From<super::SmartVideoLayerConfig> for SmartVideoLayerConfig {
        fn from(value: super::SmartVideoLayerConfig) -> Self {
            Self {
                display_duration: Ok(value.display_duration),
                video_cover_url: Ok(value.video_cover_url),
                video_url: Ok(value.video_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubtitleConfig {
        dx: Result<Option<i64>, String>,
        dy: Result<Option<i64>, String>,
        font_name: Result<super::SubtitleConfigFontName, String>,
        font_size: Result<i64, String>,
        h: Result<Option<i64>, String>,
        w: Result<Option<i64>, String>,
    }
    impl Default for SubtitleConfig {
        fn default() -> Self {
            Self {
                dx: Ok(Default::default()),
                dy: Ok(Default::default()),
                font_name: Ok(super::defaults::subtitle_config_font_name()),
                font_size: Ok(super::defaults::default_u64::<i64, 16>()),
                h: Ok(Default::default()),
                w: Ok(Default::default()),
            }
        }
    }
    impl SubtitleConfig {
        pub fn dx<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.dx = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dx: {}", e));
            self
        }
        pub fn dy<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.dy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dy: {}", e));
            self
        }
        pub fn font_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SubtitleConfigFontName>,
            T::Error: std::fmt::Display,
        {
            self.font_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_name: {}", e));
            self
        }
        pub fn font_size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.font_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_size: {}", e));
            self
        }
        pub fn h<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.h = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for h: {}", e));
            self
        }
        pub fn w<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.w = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for w: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SubtitleConfig> for super::SubtitleConfig {
        type Error = String;
        fn try_from(value: SubtitleConfig) -> Result<Self, String> {
            Ok(Self {
                dx: value.dx?,
                dy: value.dy?,
                font_name: value.font_name?,
                font_size: value.font_size?,
                h: value.h?,
                w: value.w?,
            })
        }
    }
    impl From<super::SubtitleConfig> for SubtitleConfig {
        fn from(value: super::SubtitleConfig) -> Self {
            Self {
                dx: Ok(value.dx),
                dy: Ok(value.dy),
                font_name: Ok(value.font_name),
                font_size: Ok(value.font_size),
                h: Ok(value.h),
                w: Ok(value.w),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextConfig {
        text: Result<super::TextConfigText, String>,
    }
    impl Default for TextConfig {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl TextConfig {
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextConfigText>,
            T::Error: std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TextConfig> for super::TextConfig {
        type Error = String;
        fn try_from(value: TextConfig) -> Result<Self, String> {
            Ok(Self { text: value.text? })
        }
    }
    impl From<super::TextConfig> for TextConfig {
        fn from(value: super::TextConfig) -> Self {
            Self {
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextLayerConfig {
        font_color: Result<super::TextLayerConfigFontColor, String>,
        font_name: Result<super::TextLayerConfigFontName, String>,
        font_size: Result<i64, String>,
        text_context: Result<Option<super::TextLayerConfigTextContext>, String>,
    }
    impl Default for TextLayerConfig {
        fn default() -> Self {
            Self {
                font_color: Ok(super::defaults::text_layer_config_font_color()),
                font_name: Ok(super::defaults::text_layer_config_font_name()),
                font_size: Ok(super::defaults::default_u64::<i64, 16>()),
                text_context: Ok(Default::default()),
            }
        }
    }
    impl TextLayerConfig {
        pub fn font_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextLayerConfigFontColor>,
            T::Error: std::fmt::Display,
        {
            self.font_color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_color: {}", e));
            self
        }
        pub fn font_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextLayerConfigFontName>,
            T::Error: std::fmt::Display,
        {
            self.font_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_name: {}", e));
            self
        }
        pub fn font_size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.font_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_size: {}", e));
            self
        }
        pub fn text_context<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TextLayerConfigTextContext>>,
            T::Error: std::fmt::Display,
        {
            self.text_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_context: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TextLayerConfig> for super::TextLayerConfig {
        type Error = String;
        fn try_from(value: TextLayerConfig) -> Result<Self, String> {
            Ok(Self {
                font_color: value.font_color?,
                font_name: value.font_name?,
                font_size: value.font_size?,
                text_context: value.text_context?,
            })
        }
    }
    impl From<super::TextLayerConfig> for TextLayerConfig {
        fn from(value: super::TextLayerConfig) -> Self {
            Self {
                font_color: Ok(value.font_color),
                font_name: Ok(value.font_name),
                font_size: Ok(value.font_size),
                text_context: Ok(value.text_context),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TriggerProcess {
        layer_config: Result<Option<super::SmartLayerConfig>, String>,
        reply_audios: Result<Vec<super::ReplyAudioInfo>, String>,
        reply_mode: Result<Option<super::TriggerProcessReplyMode>, String>,
        reply_order: Result<Option<super::TriggerProcessReplyOrder>, String>,
        reply_texts: Result<Vec<super::TriggerProcessReplyTextsItem>, String>,
        time_window: Result<Option<i64>, String>,
    }
    impl Default for TriggerProcess {
        fn default() -> Self {
            Self {
                layer_config: Ok(Default::default()),
                reply_audios: Ok(Default::default()),
                reply_mode: Ok(Default::default()),
                reply_order: Ok(Default::default()),
                reply_texts: Ok(Default::default()),
                time_window: Ok(Default::default()),
            }
        }
    }
    impl TriggerProcess {
        pub fn layer_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SmartLayerConfig>>,
            T::Error: std::fmt::Display,
        {
            self.layer_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layer_config: {}", e));
            self
        }
        pub fn reply_audios<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ReplyAudioInfo>>,
            T::Error: std::fmt::Display,
        {
            self.reply_audios = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reply_audios: {}", e));
            self
        }
        pub fn reply_mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TriggerProcessReplyMode>>,
            T::Error: std::fmt::Display,
        {
            self.reply_mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reply_mode: {}", e));
            self
        }
        pub fn reply_order<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TriggerProcessReplyOrder>>,
            T::Error: std::fmt::Display,
        {
            self.reply_order = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reply_order: {}", e));
            self
        }
        pub fn reply_texts<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::TriggerProcessReplyTextsItem>>,
            T::Error: std::fmt::Display,
        {
            self.reply_texts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reply_texts: {}", e));
            self
        }
        pub fn time_window<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_window = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_window: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TriggerProcess> for super::TriggerProcess {
        type Error = String;
        fn try_from(value: TriggerProcess) -> Result<Self, String> {
            Ok(Self {
                layer_config: value.layer_config?,
                reply_audios: value.reply_audios?,
                reply_mode: value.reply_mode?,
                reply_order: value.reply_order?,
                reply_texts: value.reply_texts?,
                time_window: value.time_window?,
            })
        }
    }
    impl From<super::TriggerProcess> for TriggerProcess {
        fn from(value: super::TriggerProcess) -> Self {
            Self {
                layer_config: Ok(value.layer_config),
                reply_audios: Ok(value.reply_audios),
                reply_mode: Ok(value.reply_mode),
                reply_order: Ok(value.reply_order),
                reply_texts: Ok(value.reply_texts),
                time_window: Ok(value.time_window),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideoConfig {
        bitrate: Result<i64, String>,
        clip_mode: Result<super::VideoConfigClipMode, String>,
        codec: Result<super::VideoConfigCodec, String>,
        dx: Result<Option<i64>, String>,
        dy: Result<Option<i64>, String>,
        frame_rate: Result<super::VideoConfigFrameRate, String>,
        height: Result<i64, String>,
        is_subtitle_enable: Result<bool, String>,
        subtitle_config: Result<Option<super::SubtitleConfig>, String>,
        width: Result<i64, String>,
    }
    impl Default for VideoConfig {
        fn default() -> Self {
            Self {
                bitrate: Err("no value supplied for bitrate".to_string()),
                clip_mode: Ok(super::defaults::video_config_clip_mode()),
                codec: Err("no value supplied for codec".to_string()),
                dx: Ok(Default::default()),
                dy: Ok(Default::default()),
                frame_rate: Ok(super::defaults::video_config_frame_rate()),
                height: Err("no value supplied for height".to_string()),
                is_subtitle_enable: Ok(Default::default()),
                subtitle_config: Ok(Default::default()),
                width: Err("no value supplied for width".to_string()),
            }
        }
    }
    impl VideoConfig {
        pub fn bitrate<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.bitrate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bitrate: {}", e));
            self
        }
        pub fn clip_mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::VideoConfigClipMode>,
            T::Error: std::fmt::Display,
        {
            self.clip_mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clip_mode: {}", e));
            self
        }
        pub fn codec<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::VideoConfigCodec>,
            T::Error: std::fmt::Display,
        {
            self.codec = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for codec: {}", e));
            self
        }
        pub fn dx<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.dx = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dx: {}", e));
            self
        }
        pub fn dy<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.dy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dy: {}", e));
            self
        }
        pub fn frame_rate<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::VideoConfigFrameRate>,
            T::Error: std::fmt::Display,
        {
            self.frame_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for frame_rate: {}", e));
            self
        }
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height: {}", e));
            self
        }
        pub fn is_subtitle_enable<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.is_subtitle_enable = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for is_subtitle_enable: {}",
                    e
                )
            });
            self
        }
        pub fn subtitle_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SubtitleConfig>>,
            T::Error: std::fmt::Display,
        {
            self.subtitle_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtitle_config: {}", e));
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for width: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<VideoConfig> for super::VideoConfig {
        type Error = String;
        fn try_from(value: VideoConfig) -> Result<Self, String> {
            Ok(Self {
                bitrate: value.bitrate?,
                clip_mode: value.clip_mode?,
                codec: value.codec?,
                dx: value.dx?,
                dy: value.dy?,
                frame_rate: value.frame_rate?,
                height: value.height?,
                is_subtitle_enable: value.is_subtitle_enable?,
                subtitle_config: value.subtitle_config?,
                width: value.width?,
            })
        }
    }
    impl From<super::VideoConfig> for VideoConfig {
        fn from(value: super::VideoConfig) -> Self {
            Self {
                bitrate: Ok(value.bitrate),
                clip_mode: Ok(value.clip_mode),
                codec: Ok(value.codec),
                dx: Ok(value.dx),
                dy: Ok(value.dy),
                frame_rate: Ok(value.frame_rate),
                height: Ok(value.height),
                is_subtitle_enable: Ok(value.is_subtitle_enable),
                subtitle_config: Ok(value.subtitle_config),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VideoLayerConfig {
        video_cover_url: Result<Option<super::VideoLayerConfigVideoCoverUrl>, String>,
        video_url: Result<Option<super::VideoLayerConfigVideoUrl>, String>,
    }
    impl Default for VideoLayerConfig {
        fn default() -> Self {
            Self {
                video_cover_url: Ok(Default::default()),
                video_url: Ok(Default::default()),
            }
        }
    }
    impl VideoLayerConfig {
        pub fn video_cover_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::VideoLayerConfigVideoCoverUrl>>,
            T::Error: std::fmt::Display,
        {
            self.video_cover_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_cover_url: {}", e));
            self
        }
        pub fn video_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::VideoLayerConfigVideoUrl>>,
            T::Error: std::fmt::Display,
        {
            self.video_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for video_url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<VideoLayerConfig> for super::VideoLayerConfig {
        type Error = String;
        fn try_from(value: VideoLayerConfig) -> Result<Self, String> {
            Ok(Self {
                video_cover_url: value.video_cover_url?,
                video_url: value.video_url?,
            })
        }
    }
    impl From<super::VideoLayerConfig> for VideoLayerConfig {
        fn from(value: super::VideoLayerConfig) -> Self {
            Self {
                video_cover_url: Ok(value.video_cover_url),
                video_url: Ok(value.video_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VoiceConfig {
        pitch: Result<i64, String>,
        speed: Result<i64, String>,
        voice_asset_id: Result<super::VoiceConfigVoiceAssetId, String>,
        volume: Result<i64, String>,
    }
    impl Default for VoiceConfig {
        fn default() -> Self {
            Self {
                pitch: Ok(super::defaults::default_u64::<i64, 100>()),
                speed: Ok(super::defaults::default_u64::<i64, 100>()),
                voice_asset_id: Err("no value supplied for voice_asset_id".to_string()),
                volume: Ok(super::defaults::default_u64::<i64, 140>()),
            }
        }
    }
    impl VoiceConfig {
        pub fn pitch<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.pitch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pitch: {}", e));
            self
        }
        pub fn speed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.speed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for speed: {}", e));
            self
        }
        pub fn voice_asset_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::VoiceConfigVoiceAssetId>,
            T::Error: std::fmt::Display,
        {
            self.voice_asset_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for voice_asset_id: {}", e));
            self
        }
        pub fn volume<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.volume = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for volume: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<VoiceConfig> for super::VoiceConfig {
        type Error = String;
        fn try_from(value: VoiceConfig) -> Result<Self, String> {
            Ok(Self {
                pitch: value.pitch?,
                speed: value.speed?,
                voice_asset_id: value.voice_asset_id?,
                volume: value.volume?,
            })
        }
    }
    impl From<super::VoiceConfig> for VoiceConfig {
        fn from(value: super::VoiceConfig) -> Self {
            Self {
                pitch: Ok(value.pitch),
                speed: Ok(value.speed),
                voice_asset_id: Ok(value.voice_asset_id),
                volume: Ok(value.volume),
            }
        }
    }
}
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: std::convert::TryFrom<u64>,
        <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn create_smart_live_room_req_room_type() -> super::CreateSmartLiveRoomReqRoomType {
        super::CreateSmartLiveRoomReqRoomType::Normal
    }
    pub(super) fn hit_condition_tag_operation() -> super::HitConditionTagOperation {
        super::HitConditionTagOperation::None
    }
    pub(super) fn human_position2_d_position() -> super::HumanPosition2DPosition {
        super::HumanPosition2DPosition::Middle
    }
    pub(super) fn live_event_call_back_config_auth_type() -> super::LiveEventCallBackConfigAuthType
    {
        super::LiveEventCallBackConfigAuthType::None
    }
    pub(super) fn play_policy_play_mode() -> super::PlayPolicyPlayMode {
        super::PlayPolicyPlayMode::Text
    }
    pub(super) fn play_policy_random_play_mode() -> super::PlayPolicyRandomPlayMode {
        super::PlayPolicyRandomPlayMode::ScriptItem
    }
    pub(super) fn subtitle_config_font_name() -> super::SubtitleConfigFontName {
        super::SubtitleConfigFontName("HarmonyOS_Sans_SC_Black".to_string())
    }
    pub(super) fn text_layer_config_font_color() -> super::TextLayerConfigFontColor {
        super::TextLayerConfigFontColor("16777215".to_string())
    }
    pub(super) fn text_layer_config_font_name() -> super::TextLayerConfigFontName {
        super::TextLayerConfigFontName("HarmonyOS_Sans_SC_Black".to_string())
    }
    pub(super) fn video_config_clip_mode() -> super::VideoConfigClipMode {
        super::VideoConfigClipMode::Resize
    }
    pub(super) fn video_config_frame_rate() -> super::VideoConfigFrameRate {
        super::VideoConfigFrameRate::_25
    }
}
