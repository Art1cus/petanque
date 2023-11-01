use ::yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct MatchProps {
    pub team_1: String,
    pub team_2: String,
    pub has_points: bool,
    pub round: String,
    pub match_nr: String,
}

#[function_component(MatchCard)]
pub fn match_card(props: &MatchProps) -> Html {
    let MatchProps {
        team_1,
        team_2,
        has_points,
        round,
        match_nr,
    } = props;

    html! {
        <li class="group/item relative flex items-center justify-between rounded-xl p-4 hover:bg-slate-200 bg-slate-100 shadow-md mb-3">
            <div class="flex gap-4">
                <div class="w-full text-sm leading-6">
                    <a href="#" class="font-semibold text-slate-900"><span class="absolute inset-0 rounded-xl"
                            aria-hidden="true"></span>{round} {match_nr}</a>
                    <div class="text-slate-500">{team_1}{" - "}{team_2}</div>
                </div>
            </div>
            <a href="#"
                class="group/edit invisible relative flex items-center whitespace-nowrap rounded-full py-1 pl-4 pr-3 text-sm text-slate-500 transition hover:bg-slate-200 group-hover/item:visible">
                <span class="font-semibold transition group-hover/edit:text-gray-700">{"edit"}</span>
                <svg class="mt-px h-5 w-5 text-slate-400 transition group-hover/edit:translate-x-0.5 group-hover/edit:text-slate-500"
                    viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z" clip-rule="evenodd"></path>
                </svg>
            </a>
        </li>
    }
}
