use yew::prelude::*;

#[function_component(RulesView)]
pub fn home() -> Html {
    
    html! {
        <div class="home-page">
            <div class="container page">
                <div class="card">
                    <div class="card-body">
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: center;"><span style="color: rgb(255, 217, 102);"><strong>{"SPELREGELS PETANQUE"}</strong></span></p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;"><br/></p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;"><span style="color: rgb(0, 216, 255);"><strong>{"Start van het spel"}</strong></span></p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;">{"Er wordt getost welk team het but (kleine, gele doelballetje) mag gooien. Het gegooide but mag niet tegen de rand van het petanqueveld liggen, anders moet er opnieuw gegooid worden. Daarna gooit elk team 1 bal."}</p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;"><br/></p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;"><span style="color: rgb(0, 216, 255);"><strong>{"Verloop van het spel"}</strong></span></p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;">{"Eens het but en de eerste bal van elk team zijn gegooid, moet men voor de verdere volgorde letten op de afstand van de dichtstbijzijnde bal van elk team tot het but. Het team dat het verst ligt van het but, is aan zet. Dus zolang dat team er niet in slaagt om een bal nog dichter bij het but te werpen dan de tegenstander, blijft de beurt aan hen. Werpen ze een bal dichter bij het but dan de dichtste bal van de tegenstander, dan is het de beurt aan het andere team."}</p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;">{"Het wegkaatsen van andere ballen is toegestaan. Zo gaat het verder tot alle ballen zijn gegooid. Elk team krijgt een meetlint om de afstand tot de but te controleren. Bij twijfel wordt de scheidsrechter erbij gehaald."}</p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;"><br/></p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;"><span style="color: rgb(0, 216, 255);"><strong>{"Puntentelling"}</strong></span></p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;">{"Elke bal die dichter bij het but ligt dan de dichtste bal van de tegenstander, levert een punt op. Bijvoorbeeld: twee grijze ballen liggen dichter bij het but dan de beste zwarte bal. Het team dat met grijze ballen speelt, scoort dan twee punten."}</p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;"><br/></p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;"><span style="color: rgb(0, 216, 255);"><strong>{"Einde van het spel"}</strong></span></p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;">{"Het spel eindigt vanaf wanneer een team 13 punten heeft of wanneer de scheidsrechter het einde van het spel aankondigt. Het winnende team is het team met de meeste punten."}</p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;">{"Scores dienen aan het einde van de wedstrijd meegedeeld te worden aan de scheidsrechter."}</p>
                        <p style="background: transparent;line-height: 100%;margin-top: 0.42cm;margin-bottom: 0.42cm;text-align: justify;"><span style="color: rgb(224, 102, 102);"><strong>{"Extra info voor de scheidsrechters - Wat als..."}</strong></span></p>
                        <p style="background: transparent;line-height: 100%;margin-top: 0.42cm;margin-bottom: 0.42cm;text-align: justify;"><span style="color: rgb(0, 216, 255);"><strong>{"Wat als een petanquebal tegen de rand van het veld ligt?"}</strong></span></p>
                        <p style="line-height: 115%;margin-top: 0.42cm;margin-bottom: 0.42cm;border: none;padding: 0cm;background: transparent;text-align: justify;">{"Geen probleem, de petanquebal blijft geldig als hij tegen de rand van het veld komt te liggen, gezien deze nog binnen het speelveld blijft."}</p>
                        <p style="background: transparent;line-height: 100%;margin-top: 0.42cm;margin-bottom: 0.42cm;text-align: justify;"><span style="color: rgb(0, 216, 255);"><span size="3" style="font-size:16px;"><strong>{"Wat als de ballen van beide ploegen juist even dicht bij het but liggen?"}</strong></span></span></p>
                        <p style="background: transparent;line-height: 100%;margin-top: 0.42cm;margin-bottom: 0.42cm;text-align: justify;">{"Er zijn 3 mogelijke scenario's:"}</p>
                        <ol>
                        <li>
                            <p style="background: transparent;line-height: 100%;margin-top: 0.42cm;margin-bottom: 0cm;text-align: justify;"><strong>{"Beide ploegen hebben nog ballen over"}</strong>{": de ploeg die het laatst heeft geworpen, werpt nogmaals een bal, dan de tegenstander, en vervolgens om de beurt, totdat één van de ploegen het punt heeft."}</p>
                        </li>
                        <li>
                            <p style="background: transparent;line-height: 100%;margin-bottom: 0cm;text-align: justify;"><strong>{"Slechts één ploeg heeft nog ballen over"}</strong>{": die ploeg speelt de overgebleven ballen. Als ze hiermee dichter kunnen spelen, kunnen zij hiermee nog punten scoren."}</p>
                        </li>
                        <li>
                            <p style="background: transparent;line-height: 100%;margin-bottom: 0.42cm;text-align: justify;"><strong>{"Geen van beide ploegen heeft nog ballen over"}</strong>{": het spel eindigt als onbeslist en het but wordt geworpen door de ploeg die het vorige spel heeft gewonnen."}<strong/></p>
                        </li>
                        </ol>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;"><br/></p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;"><span style="color: rgb(0, 216, 255);"><strong>{"Wat als er tijd gerokken wordt?"}</strong></span></p>
                        <p style="line-height: 115%;margin-top: 0.42cm;margin-bottom: 0.42cm;border: none;padding: 0cm;background: transparent;text-align: justify;"><em>{"Opmerking: Het is niet de bedoeling dat er standaard getimed wordt. Deze info is uitsluitend bedoeld voor het geval er echt opzettelijk veel tijd wordt gerokken."}</em></p>
                        <p style="line-height: 115%;margin-top: 0.42cm;margin-bottom: 0.42cm;border: none;padding: 0cm;background: transparent;text-align: justify;">{"In theorie heeft elke speler maximum 1 minuut de tijd om zijn bal te gooien van zodra het but gegooid is. Indien een speler zich niet aan de tijd houdt, wordt een gele kaart uitgedeeld. Indien een speler 2 gele kaarten heeft, verliest hij 1 van zijn te spelen ballen."}</p>
                        <p style="line-height: 115%;margin-bottom: 0cm;border: none;padding: 0cm;background: transparent;text-align: justify;"><span style="color: rgb(0, 216, 255);"><strong>{"Nog wat tips"}</strong></span></p>
                        <ul>
                        <li>
                            <p style="line-height: 115%;margin-top: 0.42cm;margin-bottom: 0cm;border: none;padding: 0cm;background: transparent;text-align: justify;"><strong>{"Bij het begin van een wedstrijd:"}</strong>{" controleer of de juiste 2 teams aanwezig zijn"}</p>
                        </li>
                        <li>
                            <p style="line-height: 115%;margin-bottom: 0.42cm;border: none;padding: 0cm;background: transparent;text-align: justify;"><strong>{"Bij het einde van een wedstrijd:"}</strong>{" zorg ervoor dat er bij het indienen van de scores minstens 1 persoon van elke ploeg aanwezig is die de eindstand kan bevestigen"}</p>
                        </li>
                        </ul>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;"><br/></p>
                        <p style="line-height: 115%;background: transparent;margin-bottom: 0cm;text-align: justify;"><span style="color: rgb(0, 216, 255);"><span size="3" style="font-size:17px;"><strong>{"En last but not least... geniet van de wedstrijd!"}</strong></span></span></p>
                    </div>
                </div>
            </div>
        </div>
    }
}
