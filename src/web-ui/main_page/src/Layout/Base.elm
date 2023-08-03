module Layout.Base exposing (renderLayout)

import Element
    exposing
        ( clip
        , column
        , fill
        , height
        , layout
        , width
        )
import Element.Background as Background
import Element.Font as Font
import Html exposing (Html)
import Layout.Logon.Base exposing (logonLayout)
import Layout.MainPage exposing (mainPageLayout)
import Model exposing (Model, Msg(..), SessionState(..))
import Themes.Base exposing (getTheme)


renderLayout : Model -> Html Msg
renderLayout model =
    let
        colorScheme =
            getTheme model.theme
    in
    layout
        [ width fill
        , height fill
        , clip
        , Background.color colorScheme.mainBackground
        , Font.size 14
        , Font.family [ Font.sansSerif ]
        , Font.color colorScheme.mainFontColor
        ]
    <|
        column
            [ width fill
            , height fill
            ]
            [ case model.session of
                SessionSuccess _ ->
                    mainPageLayout model

                _ ->
                    logonLayout model
            ]
