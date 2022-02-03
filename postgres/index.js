fastify = require('fastify')();
const sql = require('postgres')("postgresql://postgres:password@ip:port/db?connection_limit=5", {
    ssl: false,      // true, prefer, require, tls.connect option
    idle_timeout: 10,          // Idle connection timeout in seconds
    connect_timeout: 4,         // Connect timeout in seconds
    no_prepare: false,      // No automatic creation of prepared statements
});
let num = 0;
fastify.get('/insert', {
    response: {}
}, function (req, res) {
    const json = '{}';
    sql`insert into preferences (prefs)
        VALUES (${json});`.catch(err => {
        console.log(err);
        res.code(500).send('err');
    }).then((resp) => {
        res.code(200).send(true)
    });
});

fastify.get('/select', {
    response: {}
}, function (req, res) {
    const body = req.body;
    num++;
    sql`select *
        from preferences
        where id = ${num};`.catch(err => {
        console.log(err);
        res.code(500).send('err');
    }).then((resp) => {
        if (!res) return res.code(500).send('err');
        res.code(200).send(Array.from(resp)[0])
    });
});

fastify.listen(8086, err => {
    require('prexit')((async () => {
        await sql.end({timeout: 5})
        await fastify.close();
    }));
    if (err) throw err;
    console.log('Server listening on localhost:', fastify.server.address().port)
});