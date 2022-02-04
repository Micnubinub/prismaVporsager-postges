package main

import (
	"context"
	"fmt"
	"github.com/jackc/pgx/v4/pgxpool"
	"github.com/kataras/iris/v12"
)

var (
	pool *pgxpool.Pool
	num  int64 = 0
)

type Preference struct {
	ID    int  `json:"id" db:"id"`
	Prefs Pref `json:"prefs" db:"prefs"`
}

type Pref struct {
}

func connectToDataBase() {
	var err error
	//pool, err := pgx.Connect(context.Background(), "postgresql://postgres:development-password@localhost:12340/test?connection_limit=5")
	pool, err = pgxpool.Connect(context.Background(), "postgresql://postgres:development-password@localhost:12340/test")
	if err != nil {
		panic(err)
	}

	if err = pool.Ping(context.Background()); err != nil {
		fmt.Printf(err.Error())
		return
	}

	println("DB connected")
}

func main() {
	app := iris.New()
	api := app.Party("/")
	{
		//api.Use(iris.Compression)
		api.Get("/select", find)
		api.Get("/insert", insert)
	}
	connectToDataBase()

	app.Listen(":8086")
}

func find(ctx iris.Context) {
	num++
	rows, err := pool.Query(context.Background(), "select * from preferences where id=$1;", num)
	defer rows.Close()
	if err != nil {
		ctx.StatusCode(500)
		println(err.Error())
		ctx.ResponseWriter().Write([]byte("err"))
		return
	}
	for rows.Next() {
		pref := Preference{}

		err := rows.Scan(&pref.ID, &pref.Prefs) // order matters
		if err != nil {
			ctx.StatusCode(500)
			_, err1 := ctx.ResponseWriter().Write([]byte("err"))
			fmt.Println(err1)

			return
		}

		ctx.JSON(pref)
	}

	if err != nil {
		ctx.StatusCode(500)
		println(err.Error())
		ctx.ResponseWriter().Write([]byte("err"))
		return
	}
}

func insert(ctx iris.Context) {
	var pref Preference = Preference{
		Prefs: Pref{},
	}

	_, err := pool.Exec(context.Background(), "insert into preferences (prefs) VALUES ($1);", &pref)
	if err != nil {
		ctx.StatusCode(500)
		println(err.Error())
		ctx.ResponseWriter().Write([]byte("err"))
		return
	}

	ctx.StatusCode(iris.StatusCreated)
	ctx.Write([]byte(""))
}
