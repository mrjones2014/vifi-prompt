function fish_prompt
    set -l vi_mode
    switch "$fish_key_bindings"
        case fish_hybrid_key_bindings fish_vi_key_bindings
            set vi_mode "$fish_bind_mode"
        case '*'
            set vi_mode insert
    end
    vifi prompt --status $status --vi-mode $vi_mode
end

function fish_right_prompt
    vifi right-prompt --last-duration $CMD_DURATION
end

# Remove default mode prompt
builtin functions -e fish_mode_prompt
