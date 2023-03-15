#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;


fn init_keymap() -> Vec<&'static str> {
    vec![
        // Touchscreen settings
        "touchscreen_enabled",
        "touchscreen_preview",
        "touchscreen_info",
        "touchscreen_settings",
        "touchscreen_recordctl",
        "touchscreen_timeout",
        "touchscreen_backlight",

        // Main HTTP server settings
        "http_usessl",
        "http_port",
        "http_sport",
        
        // IP-based access control settings
        "allowips",
        "denyips",
        
        // Sources settings
        // Common systems settings
        "description",
        "multicast_ip",
        "multicast_rate_limit",
        
        // Firmware update settings
        "frmcheck_enabled",
        
        // UPNP settings
        "share_livestreams",
        "share_archive",
        "server_name",
        
        // Broadcasting settings
        "bcast_disabled",
        "streamport",
        "rtsp_port",
        
        // Common encoder settings
        "framesize",
        "autoframesize",
        "fpslimit",
        "vbitrate",
        "vencpreset",
        "vprofile",
        "vkeyframeinterval",
        "slicemode",
        "qvalue",
        "codec",
        "audio",
        "audiopreset",
        "audiobitrate",
        "audiochannels",
        
        // External encoder settings
        "type",
        "timelabel",
        "pip_layout",
        "bgcolor",
        "vgadvi",
        "keep_aspect_ratio",
        
        // Channel layouts
        "active_layout",
        
        // Stream access control
        "ac_override",
        "ac_viewerpwd",
        "ac_allowips",
        "ac_denyips",
        
        // Publish stream settings
        "publish_enabled",
        "publish_type",
        
        // RTSP announce settings (PUBLISH_TYPE = "2")
        "rtsp_url",
        "rtsp_transport",
        "rtsp_username",
        "rtsp_password",
        
        // RTMP publish settings (PUBLISH_TYPE = "6" and PUBLISH_TYPE = "7")
        "rtmp_url",
        "rtmp_stream",
        "rtmp_username",
        "rtmp_password",
        
        // Livestream publish settings (PUBLISH_TYPE = "8")
        "livestream_channel",
        "livestream_username",
        "livestream_password",
        
        // RTP/UDP publish settings (PUBLISH_TYPE = "3")
        "unicast_address",
        "unicast_aport",
        "unicast_vport",

        // MPEG-TS publish settings (PUBLISH_TYPE = "4" and PUBLISH_TYPE = "5")
        "unicast_address",
        "unicast_address",
        "unicast_mport",
        "sap",
        "sap_ip",
        "sap_group",
        "sap_channel_no",
        
        // Content metadata
        "title",
        "author",
        "copyright",
        "comment",
        
        // Recorder settings
        "rec_enabled",
        "rec_sizelimit",
        "rec_timelimit",
        "rec_format",
        "rec_prefix",
        "rec_upnp",
    ]
}




/// Check if a given string is a key.
pub fn is_key(value: &str) -> bool {
    lazy_static! {
        static ref KEYMAP: Vec<&'static str> = init_keymap();
    }
    KEYMAP.contains(&value)
}

/// Parse a given response string from the pearl API into a BTreeMap.
pub fn parse(response: &str) -> BTreeMap<&str, &str> {
    let split = response.split(" ").filter(|&x| x != "=");
    let mut result = BTreeMap::new();
    let mut key = "";
    for s in split {
        let iskey = is_key(s);

        if iskey && key == "" {
            key = s.trim();
        } else if !iskey {
            result.insert(key, s.trim());
            key = "";
        } else if iskey && key != "" {
            result.insert(key, "");
            key = s.trim();
        }

    }

    result
}

/// Create a GET querystring from a map.
pub fn create_querystring(parameters: BTreeMap<&str, &str>) -> String {
    let mut querystring = String::from("?");
    for (&key, &value) in parameters.iter() {
        querystring.push_str(&format!("{}={}&", key, value));
    }
    querystring.pop();
    querystring
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_key() {
        let key = "rec_prefix";
        let nokey = "no_key";
        assert_eq!(is_key(key), true);
        assert_eq!(is_key(nokey), false);
    }

    #[test]
    fn parse_result_map() {
        let input = "framesize = 1920x1080 autoframesize = slicemode = on audio = audiopreset = test type = ";

        let result = parse(input);
        assert_eq!(result.len(), 6);

        let mut validate = BTreeMap::new();
        validate.insert("framesize", "1920x1080");
        validate.insert("autoframesize", "");
        validate.insert("slicemode", "on");
        validate.insert("audio", "");
        validate.insert("audiopreset", "test");
        validate.insert("type", "");

        for (k, v) in result.iter() {
            assert_eq!(validate.get(k).unwrap(), v);
        }
    }

    #[test]
    fn test_querystring() {
        let mut input = BTreeMap::new();
        input.insert("slicemode", "on");
        input.insert("audiopreset", "test");

        assert_eq!(create_querystring(input), "?audiopreset=test&slicemode=on");
    }

}
