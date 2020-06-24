module Main exposing (..)

import Browser
import Html exposing (Html, div, h1, img, li, text, ul)
import Html.Attributes exposing (src)
import Http
import Json.Decode as Decode exposing (field, int, list, map3, string)



---- USER ----


type alias User =
    { first_name : String
    , last_name : String
    , id : Int
    }


user_decoder =
    map3 User
        (field "first_name" string)
        (field "first_name" string)
        (field "id" int)


user_list_decoder =
    list user_decoder


get_users =
    Http.get
        { url = "http://localhost:8000/users"
        , expect = Http.expectJson GotUsers user_list_decoder
        }



---- MODEL ----


type alias Model =
    { users : List User }


init : ( Model, Cmd Msg )
init =
    ( { users = [] }, get_users )



---- UPDATE ----


type Msg
    = GotUsers (Result Http.Error (List User))
    | NoOp


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotUsers result ->
            case result of
                Ok users ->
                    ( { model | users = users }, Cmd.none )

                Err _ ->
                    ( model, Cmd.none )

        NoOp ->
            ( model, Cmd.none )



---- VIEW ----


view : Model -> Html Msg
view model =
    div []
        [ img [ src "/logo.svg" ]
            []
        , ul []
            (List.map
                (\user -> li [] [ text user.first_name, text user.last_name ])
                model.users
            )
        ]



---- PROGRAM ----


main : Program () Model Msg
main =
    Browser.element
        { view = view
        , init = \_ -> init
        , update = update
        , subscriptions = always Sub.none
        }
