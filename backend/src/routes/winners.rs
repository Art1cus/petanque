// WITH group_wins AS (
//     SELECT
//         g.group_id,
//         gr.team_id,
//         COUNT(CASE WHEN gr.score > COALESCE(gr_opp.score, 0) THEN 1 END) AS wins,
//         SUM(gr.score) AS total_score
//     FROM
//         teams t
//     JOIN
//         groups g ON t.group_id = g.group_id
//     JOIN
//         games ga ON (t.team_id = ga.team_1_id OR t.team_id = ga.team_2_id)
//     JOIN
//         (
//             SELECT
//                 game_id,
//                 team_id,
//                 score
//             FROM
//                 game_results
//         ) gr ON ga.game_id = gr.game_id AND (t.team_id = gr.team_id)
//     LEFT JOIN
//         (
//             SELECT
//                 game_id,
//                 team_id,
//                 score
//             FROM
//                 game_results
//         ) gr_opp ON ga.game_id = gr_opp.game_id AND (t.team_id <> gr_opp.team_id)
//     GROUP BY
//         g.group_id,
//         gr.team_id
// ),
// group_ranks AS (
//     SELECT
//         group_id,
//         team_id,
//         ROW_NUMBER() OVER (PARTITION BY group_id ORDER BY wins DESC, total_score DESC) AS rank
//     FROM
//         group_wins
// )
// SELECT
//     group_id,
//     team_id
// FROM
//     group_ranks
// WHERE
//     rank = 1;
