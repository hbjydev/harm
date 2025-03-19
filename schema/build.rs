use tonic_build::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = tonic_build::configure();
    let builder = build_server(builder);

    builder.compile_protos(
        &["v0/controller/controller.proto", "v0/servers/servers.proto"],
        &["api"],
    )?;

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/lib.rs");

    Ok(())
}

fn build_server(builder: Builder) -> Builder {
    builder
        .message_attribute(
            "harm.servers.v0.ServerA2SConfig",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .message_attribute(
            "harm.servers.v0.ServerA2SConfig",
            "#[serde(rename_all = \"camelCase\")]",
        )
        .enum_attribute(
            "harm.servers.v0.RconPermission",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .enum_attribute(
            "harm.servers.v0.RconPermission",
            "#[serde(rename_all = \"lowercase\")]",
        )
        .message_attribute(
            "harm.servers.v0.ServerRconConfig",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .message_attribute(
            "harm.servers.v0.ServerRconConfig",
            "#[serde(rename_all = \"camelCase\")]",
        )
        .enum_attribute(
            "harm.servers.v0.GamePlatform",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .field_attribute(
            "harm.servers.v0.GamePlatform.PC",
            "#[serde(rename = \"PLATFORM_PC\")]",
        )
        .field_attribute(
            "harm.servers.v0.GamePlatform.XBL",
            "#[serde(rename = \"PLATFORM_XBL\")]",
        )
        .field_attribute(
            "harm.servers.v0.GamePlatform.PSN",
            "#[serde(rename = \"PLATFORM_PSN\")]",
        )
        .message_attribute(
            "harm.servers.v0.ServerGameProperties",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .message_attribute(
            "harm.servers.v0.ServerGameProperties",
            "#[serde(rename_all = \"camelCase\")]",
        )
        .message_attribute(
            "harm.servers.v0.ServerMod",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .message_attribute(
            "harm.servers.v0.ServerMod",
            "#[serde(rename_all = \"camelCase\")]",
        )
        .message_attribute(
            "harm.servers.v0.ServerGameConfig",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .message_attribute(
            "harm.servers.v0.ServerGameConfig",
            "#[serde(rename_all = \"camelCase\")]",
        )
        .field_attribute(
            "harm.servers.v0.ServerGameConfig.password",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            "harm.servers.v0.ServerGameConfig.battleye",
            "#[serde(rename = \"battlEye\")]",
        )
        .field_attribute(
            "harm.servers.v0.ServerGameConfig.von_disable_ui",
            "#[serde(rename = \"VONDisableUI\")]",
        )
        .field_attribute(
            "harm.servers.v0.ServerGameConfig.von_disable_direct_speech_ui",
            "#[serde(rename = \"VONDisableDirectSpeechUI\")]",
        )
        .field_attribute(
            "harm.servers.v0.ServerGameConfig.von_can_transmit_cross_faction",
            "#[serde(rename = \"VONCanTransmitCrossFaction\")]",
        )
        .field_attribute(
            "harm.servers.v0.ServerGameConfig.mission_header",
            "#[serde(skip_serializing_if = \"HashMap::is_empty\")]",
        )
        .message_attribute(
            "harm.servers.v0.ServerJoinQueueConfig",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .message_attribute(
            "harm.servers.v0.ServerJoinQueueConfig",
            "#[serde(rename_all = \"camelCase\")]",
        )
        .message_attribute(
            "harm.servers.v0.ServerOperatingConfig",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .message_attribute(
            "harm.servers.v0.ServerOperatingConfig",
            "#[serde(rename_all = \"camelCase\")]",
        )
        .field_attribute(
            "harm.servers.v0.ServerOperatingConfig.disable_navmesh_streaming",
            "#[serde(skip_serializing_if = \"Vec::is_empty\")]",
        )
        .field_attribute(
            "harm.servers.v0.ServerOperatingConfig.disable_ai",
            "#[serde(rename = \"disableAI\")]",
        )
        .field_attribute(
            "harm.servers.v0.ServerOperatingConfig.ai_limit",
            "#[serde(rename = \"aiLimit\")]",
        )
        .message_attribute(
            "harm.servers.v0.ServerConfig",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .message_attribute(
            "harm.servers.v0.ServerConfig",
            "#[serde(rename_all = \"camelCase\")]",
        )
        .field_attribute(
            "harm.servers.v0.ServerConfig.bind_port",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .field_attribute(
            "harm.servers.v0.ServerConfig.public_address",
            "#[serde(skip_serializing_if = \"Option::is_none\")]",
        )
        .message_attribute(
            "harm.servers.v0.ServerMetadata",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .message_attribute(
            "harm.servers.v0.ServerMetadata",
            "#[serde(rename_all = \"camelCase\")]",
        )
        .enum_attribute(
            "harm.servers.v0.ServerState",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .message_attribute(
            "harm.servers.v0.ServerStatus",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .message_attribute(
            "harm.servers.v0.Server",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
}
