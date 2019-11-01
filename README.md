# i3-groups

An adaptation of [i3-workspace-groups](https://github.com/cjbassi/i3-workspace-groups) by cjbassi.

This is a simplification, and assumes that groups are only accessed by number (0-9), and the same for workspaces within a group.

## Setup

Add the following to your i3/sway config:
```set $exec_i3_groups exec --no-startup-id i3-groups

set $switch_group Control
set $switch_group_all Alt

bindsym $mod+$switch_group+1 $exec_i3_groups focus-group 1
bindsym $mod+$switch_group+2 $exec_i3_groups focus-group 2
bindsym $mod+$switch_group+3 $exec_i3_groups focus-group 3
bindsym $mod+$switch_group+4 $exec_i3_groups focus-group 4
bindsym $mod+$switch_group+5 $exec_i3_groups focus-group 5
bindsym $mod+$switch_group+6 $exec_i3_groups focus-group 6
bindsym $mod+$switch_group+7 $exec_i3_groups focus-group 7
bindsym $mod+$switch_group+8 $exec_i3_groups focus-group 8
bindsym $mod+$switch_group+9 $exec_i3_groups focus-group 9
bindsym $mod+$switch_group+0 $exec_i3_groups focus-group 0

bindsym $mod+$switch_group_all+1 $exec_i3_groups focus-group-all 1
bindsym $mod+$switch_group_all+2 $exec_i3_groups focus-group-all 2
bindsym $mod+$switch_group_all+3 $exec_i3_groups focus-group-all 3
bindsym $mod+$switch_group_all+4 $exec_i3_groups focus-group-all 4
bindsym $mod+$switch_group_all+5 $exec_i3_groups focus-group-all 5
bindsym $mod+$switch_group_all+6 $exec_i3_groups focus-group-all 6
bindsym $mod+$switch_group_all+7 $exec_i3_groups focus-group-all 7
bindsym $mod+$switch_group_all+8 $exec_i3_groups focus-group-all 8
bindsym $mod+$switch_group_all+9 $exec_i3_groups focus-group-all 9
bindsym $mod+$switch_group_all+0 $exec_i3_groups focus-group-all 0

bindsym $mod+Shift+1 $exec_i3_groups move-container-to-workspace 1
bindsym $mod+Shift+2 $exec_i3_groups move-container-to-workspace 2
bindsym $mod+Shift+3 $exec_i3_groups move-container-to-workspace 3
bindsym $mod+Shift+4 $exec_i3_groups move-container-to-workspace 4
bindsym $mod+Shift+5 $exec_i3_groups move-container-to-workspace 5
bindsym $mod+Shift+6 $exec_i3_groups move-container-to-workspace 6
bindsym $mod+Shift+7 $exec_i3_groups move-container-to-workspace 7
bindsym $mod+Shift+8 $exec_i3_groups move-container-to-workspace 8
bindsym $mod+Shift+9 $exec_i3_groups move-container-to-workspace 9
bindsym $mod+Shift+0 $exec_i3_groups move-container-to-workspace 10

bindsym $mod+$switch_group+Shift+1 $exec_i3_groups move-container-to-group 1
bindsym $mod+$switch_group+Shift+2 $exec_i3_groups move-container-to-group 2
bindsym $mod+$switch_group+Shift+3 $exec_i3_groups move-container-to-group 3
bindsym $mod+$switch_group+Shift+4 $exec_i3_groups move-container-to-group 4
bindsym $mod+$switch_group+Shift+5 $exec_i3_groups move-container-to-group 5
bindsym $mod+$switch_group+Shift+6 $exec_i3_groups move-container-to-group 6
bindsym $mod+$switch_group+Shift+7 $exec_i3_groups move-container-to-group 7
bindsym $mod+$switch_group+Shift+8 $exec_i3_groups move-container-to-group 8
bindsym $mod+$switch_group+Shift+9 $exec_i3_groups move-container-to-group 9
bindsym $mod+$switch_group+Shift+0 $exec_i3_groups move-container-to-group 0

bindsym $mod+1 $exec_i3_groups focus-workspace 1
bindsym $mod+2 $exec_i3_groups focus-workspace 2
bindsym $mod+3 $exec_i3_groups focus-workspace 3
bindsym $mod+4 $exec_i3_groups focus-workspace 4
bindsym $mod+5 $exec_i3_groups focus-workspace 5
bindsym $mod+6 $exec_i3_groups focus-workspace 6
bindsym $mod+7 $exec_i3_groups focus-workspace 7
bindsym $mod+8 $exec_i3_groups focus-workspace 8
bindsym $mod+9 $exec_i3_groups focus-workspace 9
bindsym $mod+0 $exec_i3_groups focus-workspace 10
```
