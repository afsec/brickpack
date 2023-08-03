module UIC.Hello exposing (helloUi)

import Element
    exposing
        ( centerX
        , centerY
        , column
        , el
        , height
        , image
        , px
        , row
        , spacingXY
        , text
        , width
        )
import Element.Font as Font
import Model exposing (Model, Msg(..))
import UIC.Buttons exposing (simpleButton)


helloUi : Model -> Element.Element Msg
helloUi model =
    column
        [ centerX
        , centerY
        , spacingXY 0 10

        -- , Element.explain Debug.todo
        ]
        [ image
            [ width <| px 150
            , height <| px 150
            , centerX
            ]
            { src = "/logo.svg"
            , description = "Some image"
            }
        , row [ centerX ]
            [ el [] <|
                text "Current theme: "
            , el
                [ Font.bold ]
              <|
                text model.theme
            ]
        , el [ centerX, Font.size 24 ] <| text "Your Elm App is working!"
        , simpleButton model "toggle theme" <| Just ToggleTheme
        ]
