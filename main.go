package main

import (
  "flag"
  "fmt"
  "os"

  "github.com/dghubble/go-twitter/twitter"
  "github.com/dghubble/oauth1"
)

func createClient() *twitter.Client {
  APIToken := os.Getenv("API_KEY")
  APITokenSecret := os.Getenv("API_KEY_SECRET")
  consumerToken := os.Getenv("CONSUMER_KEY")
  consumerTokenSecret := os.Getenv("CONSUMER_KEY_SECRET")

  config := oauth1.NewConfig(APIToken, APITokenSecret)
  token := oauth1.NewToken(consumerToken, consumerTokenSecret)
  httpClient := config.Client(oauth1.NoContext, token)

  return twitter.NewClient(httpClient)
}

func main() {
  var (
    c = flag.Int("batch", 50, "Number of tweets to fetch")
    v = flag.Bool("delete", false, "if dry-run is specified, it runs on dry-run mode")
    s = flag.Bool("text", false, "show messages of tweets")
    t = flag.Int("threshold", 5, "Minimum favorite count not to be deleted")
    m = flag.Int("iter", 10, "Number of iterate next cursor")
    i = flag.Int64("maxId", 8888888888888888888, "Maximum number of tweet ID")
  )
  flag.Parse()

  client := createClient()
  screenName := os.Getenv("SCREEN_NAME")

  // constant for MAX TWEET ID
  var maxID int64 = *i
  var messagePrefix string

  for i := 0; i < *m; i++ {

    userTimelineParams := &twitter.UserTimelineParams{
      ScreenName: screenName,
      Count:      *c,
      MaxID:      maxID,
    }

    tweets, _, e := client.Timelines.UserTimeline(userTimelineParams)
    if e != nil {
      fmt.Println(e)
      fmt.Println("Error")
      return
    }

    fmt.Println("Current MaxID: ", maxID)
    for index := range tweets {
      tweet := tweets[index]

      if maxID > tweet.ID {
        maxID = tweet.ID
      }

      if tweet.FavoriteCount >= *t {
        continue
      }

      if *v {
        messagePrefix = "Delete: "
        statusDestroyParams := &twitter.StatusDestroyParams{}
        client.Statuses.Destroy(tweet.ID, statusDestroyParams)

      } else {
        messagePrefix = "[dry-run] Delete: "
      }

      fmt.Println(messagePrefix, tweet.ID, tweet.FavoriteCount)
      if *s {
        fmt.Println(tweet.Text)
      }
    }
  }
}
