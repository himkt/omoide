package main

import (
	"fmt"
	"os"

	"github.com/dghubble/go-twitter/twitter"
	"github.com/dghubble/oauth1"
)

func main() {

	APIToken := os.Getenv("API_KEY")
	APITokenSecret := os.Getenv("API_KEY_SECRET")
	consumerToken := os.Getenv("CONSUMER_KEY")
	consumerTokenSecret := os.Getenv("CONSUMER_KEY_SECRET")

	config := oauth1.NewConfig(APIToken, APITokenSecret)
	token := oauth1.NewToken(consumerToken, consumerTokenSecret)
	httpClient := config.Client(oauth1.NoContext, token)

	// Twitter client
	client := twitter.NewClient(httpClient)

  userShowParams := &twitter.UserShowParams{ScreenName: "himkt"}
  user, _, _ := client.Users.Show(userShowParams)
  userID := user.ID
  fmt.Println(userID)

	// Home Timeline
	// tweets, _, _ := client.Timelines.HomeTimeline(&twitter.HomeTimelineParams{
	// 	Count: 20,
	// })

	// fmt.Print(tweets)
}
