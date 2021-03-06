port module Main exposing (main)

import Browser as Bro
import Html exposing (Html, button, div, h1, input, text)
import Html.Attributes exposing (placeholder, value)
import Html.Events exposing (onClick, onInput)


type alias Model =
    { received : String, sent : String }


type Msg
    = Send
    | UpdateMsg String
    | Receive String


port sender : String -> Cmd msg


port receiver : (String -> msg) -> Sub msg


main : Program () Model Msg
main =
    Bro.element { init = init, view = view, update = update, subscriptions = subscription }


init : () -> ( Model, Cmd Msg )
init flags =
    ( { received = "test message", sent = "" }, Cmd.none )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Send ->
            ( model, sender model.sent )

        Receive receivedMsg ->
            ( { model | received = receivedMsg }, Cmd.none )

        UpdateMsg updateMsg ->
            ( { model | sent = updateMsg }, Cmd.none )


view : Model -> Html Msg
view model =
    div []
        [ h1 [] [ text "Communication experiment between elm and backend rust appliction over gRPC" ]
        , input [ value model.sent, onInput UpdateMsg, placeholder "this message will be sent" ] []
        , button [ onClick Send ] [ text "Send to remote rust-app" ]
        , div [] [ text "Following line displays the message from remote rust app" ]
        , div [] [ text model.received ]
        ]


subscription : Model -> Sub Msg
subscription _ =
    receiver Receive
