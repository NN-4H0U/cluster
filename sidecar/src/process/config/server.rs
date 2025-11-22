//! https://github.com/rcsoccersim/rcssserver/blob/master/src/playerparam.cpp

use crate::create_config;

create_config! (ServerConfig, "server", {
    version: &'static str,
    catch_ban_cycle: i32,
    clang_advice_win: i32,
    clang_define_win: i32,
    clang_del_win: i32,
    clang_info_win: i32,
    clang_mess_delay: i32,
    clang_mess_per_cycle: i32,
    clang_meta_win: i32,
    clang_rule_win: i32,
    clang_win_size: i32,
    coach_port: u16,
    connect_wait: i32,
    drop_ball_time: i32,
    extra_half_time: i32,
    foul_cycles: i32,
    freeform_send_period: i32,
    freeform_wait_period: i32,
    game_log_compression: i32,
    game_log_version: i32,
    game_over_wait: i32,
    goalie_max_moves: i32,
    half_time: i32,
    hear_decay: i32,
    hear_inc: i32,
    hear_max: i32,
    illegal_defense_duration: i32,
    illegal_defense_number: i32,
    keepaway_start: i32,
    kick_off_wait: i32,
    max_goal_kicks: i32,
    max_monitors: i32,
    nr_extra_halfs: i32,
    nr_normal_halfs: i32,
    olcoach_port: u16,
    pen_before_setup_wait: i32,
    pen_max_extra_kicks: i32,
    pen_nr_kicks: i32,
    pen_ready_wait: i32,
    pen_setup_wait: i32,
    pen_taken_wait: i32,
    point_to_ban: i32,
    point_to_duration: i32,
    port: u16,
    recv_step: i32,
    say_coach_cnt_max: i32,
    say_coach_msg_size: i32,
    say_msg_size: i32,
    send_step: i32,
    send_vi_step: i32,
    sense_body_step: i32,
    simulator_step: i32,
    slow_down_factor: i32,
    start_goal_l: i32,
    start_goal_r: i32,
    synch_micro_sleep: i32,
    synch_offset: i32,
    synch_see_offset: i32,
    tackle_cycles: i32,
    text_log_compression: i32,
    auto_mode: bool,
    back_passes: bool,
    coach: bool,
    coach_w_referee: bool,
    forbid_kick_off_offside: bool,
    free_kick_faults: bool,
    fullstate_l: bool,
    fullstate_r: bool,
    game_log_dated: bool,
    game_log_fixed: bool,
    game_logging: bool,
    golden_goal: bool,
    keepaway: bool,
    keepaway_log_dated: bool,
    keepaway_log_fixed: bool,
    keepaway_logging: bool,
    log_times: bool,
    old_coach_hear: bool,
    pen_allow_mult_kicks: bool,
    pen_coach_moves_players: bool,
    pen_random_winner: bool,
    penalty_shoot_outs: bool,
    profile: bool,
    proper_goal_kicks: bool,
    record_messages: bool,
    send_comms: bool,
    synch_mode: bool,
    team_actuator_noise: bool,
    text_log_dated: bool,
    text_log_fixed: bool,
    text_logging: bool,
    use_offside: bool,
    verbose: bool,
    wind_none: bool,
    wind_random: bool,
    audio_cut_dist: f64,
    back_dash_rate: f64,
    ball_accel_max: f64,
    ball_decay: f64,
    ball_rand: f64,
    ball_size: f64,
    ball_speed_max: f64,
    ball_stuck_area: f64,
    ball_weight: f64,
    catch_probability: f64,
    catchable_area_l: f64,
    catchable_area_w: f64,
    ckick_margin: f64,
    control_radius: f64,
    dash_angle_step: f64,
    dash_power_rate: f64,
    dist_noise_rate: f64,
    effort_dec: f64,
    effort_dec_thr: f64,
    effort_inc: f64,
    effort_inc_thr: f64,
    effort_init: f64,
    effort_min: f64,
    extra_stamina: f64,
    focus_dist_noise_rate: f64,
    foul_detect_probability: f64,
    foul_exponent: f64,
    goal_width: f64,
    illegal_defense_dist_x: f64,
    illegal_defense_width: f64,
    inertia_moment: f64,
    keepaway_length: f64,
    keepaway_width: f64,
    kick_power_rate: f64,
    kick_rand: f64,
    kick_rand_factor_l: f64,
    kick_rand_factor_r: f64,
    kickable_margin: f64,
    land_dist_noise_rate: f64,
    land_focus_dist_noise_rate: f64,
    max_back_tackle_power: f64,
    max_catch_angle: f64,
    max_dash_angle: f64,
    max_dash_power: f64,
    max_tackle_power: f64,
    maxmoment: f64,
    maxneckang: f64,
    maxneckmoment: f64,
    maxpower: f64,
    min_catch_angle: f64,
    min_dash_angle: f64,
    min_dash_power: f64,
    minmoment: f64,
    minneckang: f64,
    minneckmoment: f64,
    minpower: f64,
    offside_active_area_size: f64,
    offside_kick_margin: f64,
    pen_dist_x: f64,
    pen_max_goalie_dist_x: f64,
    player_accel_max: f64,
    player_decay: f64,
    player_rand: f64,
    player_size: f64,
    player_speed_max: f64,
    player_speed_max_min: f64,
    player_weight: f64,
    prand_factor_l: f64,
    prand_factor_r: f64,
    quantize_step: f64,
    quantize_step_l: f64,
    recover_dec: f64,
    recover_dec_thr: f64,
    recover_init: f64,
    recover_min: f64,
    red_card_probability: f64,
    side_dash_rate: f64,
    slowness_on_top_for_left_team: f64,
    slowness_on_top_for_right_team: f64,
    stamina_capacity: f64,
    stamina_inc_max: f64,
    stamina_max: f64,
    stopped_ball_vel: f64,
    tackle_back_dist: f64,
    tackle_dist: f64,
    tackle_exponent: f64,
    tackle_power_rate: f64,
    tackle_rand_factor: f64,
    tackle_width: f64,
    visible_angle: f64,
    visible_distance: f64,
    wind_ang: f64,
    wind_dir: f64,
    wind_force: f64,
    wind_rand: f64,
    coach_msg_file: &'static str,
    fixed_teamname_l: &'static str,
    fixed_teamname_r: &'static str,
    game_log_dir: &'static str,
    game_log_fixed_name: &'static str,
    keepaway_log_dir: &'static str,
    keepaway_log_fixed_name: &'static str,
    landmark_file: &'static str,
    log_date_format: &'static str,
    team_l_start: &'static str,
    team_r_start: &'static str,
    text_log_dir: &'static str,
    text_log_fixed_name: &'static str,
});

// impl Default for ServerConfig {
//     fn default() -> Self {
//         Self {
//             /* server Configuration file */
//
//             // server::version
//             version: "19.0.0",
//
//             // server::catch_ban_cycle
//             catch_ban_cycle: 5,
//
//             // server::clang_advice_win
//             clang_advice_win: 1,
//
//             // server::clang_define_win
//             clang_define_win: 1,
//
//             // server::clang_del_win
//             clang_del_win: 1,
//
//             // server::clang_info_win
//             clang_info_win: 1,
//
//             // server::clang_mess_delay
//             clang_mess_delay: 50,
//
//             // server::clang_mess_per_cycle
//             clang_mess_per_cycle: 1,
//
//             // server::clang_meta_win
//             clang_meta_win: 1,
//
//             // server::clang_rule_win
//             clang_rule_win: 1,
//
//             // server::clang_win_size
//             clang_win_size: 300,
//
//             // server::coach_port
//             coach_port: 6001,
//
//             // server::connect_wait
//             connect_wait: 300,
//
//             // server::drop_ball_time
//             drop_ball_time: 100,
//
//             // server::extra_half_time
//             extra_half_time: 100,
//
//             // server::foul_cycles
//             foul_cycles: 5,
//
//             // server::freeform_send_period
//             freeform_send_period: 20,
//
//             // server::freeform_wait_period
//             freeform_wait_period: 600,
//
//             // server::game_log_compression
//             game_log_compression: 0,
//
//             // server::game_log_version
//             game_log_version: 6,
//
//             // server::game_over_wait
//             game_over_wait: 100,
//
//             // server::goalie_max_moves
//             goalie_max_moves: 2,
//
//             // server::half_time
//             half_time: 300,
//
//             // server::hear_decay
//             hear_decay: 1,
//
//             // server::hear_inc
//             hear_inc: 1,
//
//             // server::hear_max
//             hear_max: 1,
//
//             // server::illegal_defense_duration
//             illegal_defense_duration: 20,
//
//             // server::illegal_defense_number
//             /* if be 0, illegal defense rule will be disable */
//             illegal_defense_number: 0,
//
//             // server::keepaway_start
//             keepaway_start: -1,
//
//             // server::kick_off_wait
//             kick_off_wait: 100,
//
//             // server::max_goal_kicks
//             max_goal_kicks: 3,
//
//             // server::max_monitors
//             max_monitors: -1,
//
//             // server::nr_extra_halfs
//             /* Number if extra-time periods in a game if it is drawn */
//             nr_extra_halfs: 2,
//
//             // server::nr_normal_halfs
//             /* Number of normal halfs in a game */
//             nr_normal_halfs: 2,
//
//             // server::olcoach_port
//             olcoach_port: 6002,
//
//             // server::pen_before_setup_wait
//             pen_before_setup_wait: 10,
//
//             // server::pen_max_extra_kicks
//             pen_max_extra_kicks: 5,
//
//             // server::pen_nr_kicks
//             pen_nr_kicks: 5,
//
//             // server::pen_ready_wait
//             pen_ready_wait: 10,
//
//             // server::pen_setup_wait
//             pen_setup_wait: 70,
//
//             // server::pen_taken_wait
//             pen_taken_wait: 150,
//
//             // server::point_to_ban
//             point_to_ban: 5,
//
//             // server::point_to_duration
//             point_to_duration: 20,
//
//             // server::port
//             port: 6000,
//
//             // server::recv_step
//             recv_step: 10,
//
//             // server::say_coach_cnt_max
//             say_coach_cnt_max: 128,
//
//             // server::say_coach_msg_size
//             say_coach_msg_size: 128,
//
//             // server::say_msg_size
//             say_msg_size: 10,
//
//             // server::send_step
//             send_step: 150,
//
//             // server::send_vi_step
//             send_vi_step: 100,
//
//             // server::sense_body_step
//             sense_body_step: 100,
//
//             // server::simulator_step
//             simulator_step: 100,
//
//             // server::slow_down_factor
//             slow_down_factor: 1,
//
//             // server::start_goal_l
//             start_goal_l: 0,
//
//             // server::start_goal_r
//             start_goal_r: 0,
//
//             // server::synch_micro_sleep
//             synch_micro_sleep: 1,
//
//             // server::synch_offset
//             synch_offset: 60,
//
//             // server::synch_see_offset
//             synch_see_offset: 0,
//
//             // server::tackle_cycles
//             tackle_cycles: 10,
//
//             // server::text_log_compression
//             text_log_compression: 0,
//
//             // server::auto_mode
//             auto_mode: false,
//
//             // server::back_passes
//             back_passes: true,
//
//             // server::coach
//             coach: false,
//
//             // server::coach_w_referee
//             coach_w_referee: false,
//
//             // server::forbid_kick_off_offside
//             forbid_kick_off_offside: true,
//
//             // server::free_kick_faults
//             free_kick_faults: true,
//
//             // server::fullstate_l
//             fullstate_l: false,
//
//             // server::fullstate_r
//             fullstate_r: false,
//
//             // server::game_log_dated
//             game_log_dated: true,
//
//             // server::game_log_fixed
//             game_log_fixed: false,
//
//             // server::game_logging
//             game_logging: true,
//
//             // server::golden_goal
//             golden_goal: false,
//
//             // server::keepaway
//             keepaway: false,
//
//             // server::keepaway_log_dated
//             keepaway_log_dated: true,
//
//             // server::keepaway_log_fixed
//             keepaway_log_fixed: false,
//
//             // server::keepaway_logging
//             keepaway_logging: true,
//
//             // server::log_times
//             log_times: false,
//
//             // server::old_coach_hear
//             old_coach_hear: false,
//
//             // server::pen_allow_mult_kicks
//             /* Turn on to allow dribbling in penalty shootouts */
//             pen_allow_mult_kicks: true,
//
//             // server::pen_coach_moves_players
//             /* Turn on to have the server automatically position players for
//             peanlty shootouts */
//             pen_coach_moves_players: true,
//
//             // server::pen_random_winner
//             pen_random_winner: false,
//
//             // server::penalty_shoot_outs
//             /* Set to true to enable penalty shootouts after normal time and extra
//             time if the game is drawn.
//             To have the game go straight into penalty shoot outs, set this to true
//             and nr_normal_halfs and nr_extra_halfs to 0 */
//             penalty_shoot_outs: true,
//
//             // server::profile
//             profile: false,
//
//             // server::proper_goal_kicks
//             proper_goal_kicks: false,
//
//             // server::record_messages
//             record_messages: false,
//
//             // server::send_comms
//             send_comms: false,
//
//             // server::synch_mode
//             synch_mode: false,
//
//             // server::team_actuator_noise
//             team_actuator_noise: false,
//
//             // server::text_log_dated
//             text_log_dated: true,
//
//             // server::text_log_fixed
//             text_log_fixed: false,
//
//             // server::text_logging
//             text_logging: true,
//
//             // server::use_offside
//             use_offside: true,
//
//             // server::verbose
//             verbose: false,
//
//             // server::wind_none
//             wind_none: false,
//
//             // server::wind_random
//             wind_random: false,
//
//             // server::audio_cut_dist
//             audio_cut_dist: 50.0,
//
//             // server::back_dash_rate
//             back_dash_rate: 0.7,
//
//             // server::ball_accel_max
//             ball_accel_max: 2.7,
//
//             // server::ball_decay
//             ball_decay: 0.94,
//
//             // server::ball_rand
//             ball_rand: 0.05,
//
//             // server::ball_size
//             ball_size: 0.085,
//
//             // server::ball_speed_max
//             ball_speed_max: 3.0,
//
//             // server::ball_stuck_area
//             ball_stuck_area: 3.0,
//
//             // server::ball_weight
//             ball_weight: 0.2,
//
//             // server::catch_probability
//             catch_probability: 1.0,
//
//             // server::catchable_area_l
//             catchable_area_l: 1.2,
//
//             // server::catchable_area_w
//             catchable_area_w: 1.0,
//
//             // server::ckick_margin
//             ckick_margin: 1.0,
//
//             // server::control_radius
//             control_radius: 2.0,
//
//             // server::dash_angle_step
//             dash_angle_step: 1.0,
//
//             // server::dash_power_rate
//             dash_power_rate: 0.006,
//
//             // server::dist_noise_rate
//             dist_noise_rate: 0.0125,
//
//             // server::effort_dec
//             effort_dec: 0.005,
//
//             // server::effort_dec_thr
//             effort_dec_thr: 0.3,
//
//             // server::effort_inc
//             effort_inc: 0.01,
//
//             // server::effort_inc_thr
//             effort_inc_thr: 0.6,
//
//             // server::effort_init
//             effort_init: 1.0,
//
//             // server::effort_min
//             effort_min: 0.6,
//
//             // server::extra_stamina
//             extra_stamina: 50.0,
//
//             // server::focus_dist_noise_rate
//             focus_dist_noise_rate: 0.0125,
//
//             // server::foul_detect_probability
//             foul_detect_probability: 0.5,
//
//             // server::foul_exponent
//             foul_exponent: 10.0,
//
//             // server::goal_width
//             /* The width of the goals */
//             goal_width: 14.02,
//
//             // server::illegal_defense_dist_x
//             illegal_defense_dist_x: 16.5,
//
//             // server::illegal_defense_width
//             illegal_defense_width: 40.32,
//
//             // server::inertia_moment
//             inertia_moment: 5.0,
//
//             // server::keepaway_length
//             keepaway_length: 20.0,
//
//             // server::keepaway_width
//             keepaway_width: 20.0,
//
//             // server::kick_power_rate
//             kick_power_rate: 0.027,
//
//             // server::kick_rand
//             kick_rand: 0.1,
//
//             // server::kick_rand_factor_l
//             kick_rand_factor_l: 1.0,
//
//             // server::kick_rand_factor_r
//             kick_rand_factor_r: 1.0,
//
//             // server::kickable_margin
//             kickable_margin: 0.7,
//
//             // server::land_dist_noise_rate
//             land_dist_noise_rate: 0.00125,
//
//             // server::land_focus_dist_noise_rate
//             land_focus_dist_noise_rate: 0.00125,
//
//             // server::max_back_tackle_power
//             max_back_tackle_power: 0.0,
//
//             // server::max_catch_angle
//             max_catch_angle: 90.0,
//
//             // server::max_dash_angle
//             max_dash_angle: 180.0,
//
//             // server::max_dash_power
//             max_dash_power: 100.0,
//
//             // server::max_tackle_power
//             max_tackle_power: 100.0,
//
//             // server::maxmoment
//             maxmoment: 180.0,
//
//             // server::maxneckang
//             maxneckang: 90.0,
//
//             // server::maxneckmoment
//             maxneckmoment: 180.0,
//
//             // server::maxpower
//             maxpower: 100.0,
//
//             // server::min_catch_angle
//             min_catch_angle: -90.0,
//
//             // server::min_dash_angle
//             min_dash_angle: -180.0,
//
//             // server::min_dash_power
//             min_dash_power: 0.0,
//
//             // server::minmoment
//             minmoment: -180.0,
//
//             // server::minneckang
//             minneckang: -90.0,
//
//             // server::minneckmoment
//             minneckmoment: -180.0,
//
//             // server::minpower
//             minpower: -100.0,
//
//             // server::offside_active_area_size
//             offside_active_area_size: 2.5,
//
//             // server::offside_kick_margin
//             offside_kick_margin: 9.15,
//
//             // server::pen_dist_x
//             pen_dist_x: 42.5,
//
//             // server::pen_max_goalie_dist_x
//             pen_max_goalie_dist_x: 14.0,
//
//             // server::player_accel_max
//             /* The max acceleration of players */
//             player_accel_max: 1.0,
//
//             // server::player_decay
//             /* Players speed decay rate */
//             player_decay: 0.4,
//
//             // server::player_rand
//             /* Player random movement factor */
//             player_rand: 0.1,
//
//             // server::player_size
//             /* The size of the default player */
//             player_size: 0.3,
//
//             // server::player_speed_max
//             /* The max speed of players */
//             player_speed_max: 1.05,
//
//             // server::player_speed_max_min
//             /* The minumum value of the max speed of players */
//             player_speed_max_min: 0.75,
//
//             // server::player_weight
//             /* The weight of the player */
//             player_weight: 60.0,
//
//             // server::prand_factor_l
//             prand_factor_l: 1.0,
//
//             // server::prand_factor_r
//             prand_factor_r: 1.0,
//
//             // server::quantize_step
//             quantize_step: 0.1,
//
//             // server::quantize_step_l
//             quantize_step_l: 0.01,
//
//             // server::recover_dec
//             recover_dec: 0.002,
//
//             // server::recover_dec_thr
//             recover_dec_thr: 0.3,
//
//             // server::recover_init
//             /* The intial recovery value for players */
//             recover_init: 1.0,
//
//             // server::recover_min
//             recover_min: 0.5,
//
//             // server::red_card_probability
//             red_card_probability: 0.0,
//
//             // server::side_dash_rate
//             side_dash_rate: 0.4,
//
//             // server::slowness_on_top_for_left_team
//             slowness_on_top_for_left_team: 1.0,
//
//             // server::slowness_on_top_for_right_team
//             slowness_on_top_for_right_team: 1.0,
//
//             // server::stamina_capacity
//             stamina_capacity: 130600.0,
//
//             // server::stamina_inc_max
//             /* The maximum player stamina increament */
//             stamina_inc_max: 45.0,
//
//             // server::stamina_max
//             /* The maximum stamina of players */
//             stamina_max: 8000.0,
//
//             // server::stopped_ball_vel
//             stopped_ball_vel: 0.01,
//
//             // server::tackle_back_dist
//             tackle_back_dist: 0.0,
//
//             // server::tackle_dist
//             tackle_dist: 2.0,
//
//             // server::tackle_exponent
//             tackle_exponent: 6.0,
//
//             // server::tackle_power_rate
//             tackle_power_rate: 0.027,
//
//             // server::tackle_rand_factor
//             tackle_rand_factor: 2.0,
//
//             // server::tackle_width
//             tackle_width: 1.25,
//
//             // server::visible_angle
//             visible_angle: 90.0,
//
//             // server::visible_distance
//             visible_distance: 3.0,
//
//             // server::wind_ang
//             wind_ang: 0.0,
//
//             // server::wind_dir
//             wind_dir: 0.0,
//
//             // server::wind_force
//             wind_force: 0.0,
//
//             // server::wind_rand
//             wind_rand: 0.0,
//
//             // server::coach_msg_file
//             coach_msg_file: "",
//
//             // server::fixed_teamname_l
//             fixed_teamname_l: "",
//
//             // server::fixed_teamname_r
//             fixed_teamname_r: "",
//
//             // server::game_log_dir
//             game_log_dir: "./",
//
//             // server::game_log_fixed_name
//             game_log_fixed_name: "rcssserver",
//
//             // server::keepaway_log_dir
//             keepaway_log_dir: "./",
//
//             // server::keepaway_log_fixed_name
//             keepaway_log_fixed_name: "rcssserver",
//
//             // server::landmark_file
//             landmark_file: "~/.rcssserver-landmark.xml",
//
//             // server::log_date_format
//             log_date_format: "%Y%m%d%H%M%S-",
//
//             // server::team_l_start
//             team_l_start: "",
//
//             // server::team_r_start
//             team_r_start: "",
//
//             // server::text_log_dir
//             text_log_dir: "./",
//
//             // server::text_log_fixed_name
//             text_log_fixed_name: "rcssserver",
//         }
//     }
// }
