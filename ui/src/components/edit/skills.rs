use dioxus::prelude::*;
use types::mechanics::Skill;

use crate::components::edit::Checkbox;

#[component]
pub fn SkillMultiSelect(list: Signal<Vec<Skill>>) -> Element {
    use Skill::*;
    // region: Signal
    let mut acrobatics = use_signal(|| list().contains(&Acrobatics));
    let mut animal_handling = use_signal(|| list().contains(&AnimalHandling));
    let mut arcana = use_signal(|| list().contains(&Arcana));
    let mut athletics = use_signal(|| list().contains(&Athletics));
    let mut deception = use_signal(|| list().contains(&Deception));
    let mut history = use_signal(|| list().contains(&History));
    let mut insight = use_signal(|| list().contains(&Insight));
    let mut intimidation = use_signal(|| list().contains(&Intimidation));
    let mut investigation = use_signal(|| list().contains(&Investigation));
    let mut medicine = use_signal(|| list().contains(&Medicine));
    let mut nature = use_signal(|| list().contains(&Nature));
    let mut perception = use_signal(|| list().contains(&Perception));
    let mut performance = use_signal(|| list().contains(&Performance));
    let mut persuasion = use_signal(|| list().contains(&Persuasion));
    let mut religion = use_signal(|| list().contains(&Religion));
    let mut sleight_of_hand = use_signal(|| list().contains(&SleightOfHand));
    let mut stealth = use_signal(|| list().contains(&Stealth));
    let mut survival = use_signal(|| list().contains(&Survival));

    macro_rules! checkbox {
        ($signal:ident, $skill:ident) => {
            rsx! {
                Checkbox {
                    name: $skill.to_string(),
                    checked: $signal(),
                    onchange: move |checked| {
                        let mut l = list();
                        if checked {
                            l.push($skill);
                        } else {
                            l.retain(|s| *s != $skill);
                        }
                        list.set(l);
                        $signal.set(checked);
                    }
                },
            }
        };
    }

    rsx! {
        div {
            {checkbox!(acrobatics, Acrobatics)}
            {checkbox!(animal_handling, AnimalHandling)}
            {checkbox!(arcana, Arcana)}
            {checkbox!(athletics, Athletics)}
            {checkbox!(deception, Deception)}
            {checkbox!(history, History)}
            {checkbox!(insight, Insight)}
            {checkbox!(intimidation, Intimidation)}
            {checkbox!(investigation, Investigation)}
            {checkbox!(medicine, Medicine)}
            {checkbox!(nature, Nature)}
            {checkbox!(perception, Perception)}
            {checkbox!(performance, Performance)}
            {checkbox!(persuasion, Persuasion)}
            {checkbox!(religion, Religion)}
            {checkbox!(sleight_of_hand, SleightOfHand)}
            {checkbox!(stealth, Stealth)}
            {checkbox!(survival, Survival)}
        }
    }
}
