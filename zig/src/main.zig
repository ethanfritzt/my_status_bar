const std = @import("std");

const BatteryInfo = struct {
    battery_path: []const u8,
    total_charge: []const u8,
    current_charge: []const u8,
    status: []const u8
};

const StatusLineBlock = struct {
    full_text: []const u8,
    short_text: ?[]const u8 = null,
    color: ?[]const u8 = null,
    background: ?[]const u8 = null,
    border: ?[]const u8 = null,
    border_top: ?i32 = null,
    border_bottom: ?i32 = null,
    border_left: ?i32 = null,
    border_right: ?i32 = null,
    min_width: ?i32 = null,
    @"align": ?[]const u8 = null, // align is a reserved keyword, this escapes it for JSON serialization use
    name: ?[]const u8 = null,
    instance: ?[]const u8 = null,
    urgent: ?bool = null,
    separator: ?bool = null,
    separator_block_width: ?i32 = null,
    markup: ?[]const u8 = null,
};

// returns the path of the battery
// ex. /sys/class/power_supply/BAT1
fn find_battery() []const u8 {

}

// returns battery level parsed through uevent
fn get_battery_level []const u8 {

}

pub fn main() !void {
    const allocater = std.heap.page_allocator;

    const s: StatusLineBlock = .{
        .full_text = "test"
    };
    var string: std.Io.Writer.Allocating = .init(allocater);
    defer string.deinit();

    try string.writer.print("{f}", .{std.json.fmt(s, .{ .emit_null_optional_fields = false })});

    std.debug.print("{s}\n", .{string.written()});
}
