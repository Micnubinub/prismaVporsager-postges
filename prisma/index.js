fastify = require('fastify')();
const {PrismaClient} = require('@prisma/client');
const prisma = new PrismaClient();
let num = 0;
fastify.get('/insert', {
    response: {}
}, function (req, res) {
    const body = req.body;
    prisma.preference.create({
        data: {prefs: {}}
    }).catch(err => {
        console.log(err);
        res.code(500).send('err');
    }).then((resp) => {
        res.code(200).send(true)
    });
});

fastify.get('/select', {
    response: {}
}, function (req, res) {
    num++;
    prisma.preference.findUnique({
        where: {
            id: num
        }
    }).catch(err => {
        console.log(err);
        res.code(500).send('err');
    }).then((resp) => {
        if (!res) return res.code(500).send('err');
        res.code(200).send(resp);
    });
});

fastify.listen(8086, err => {
    require('prexit')((async () => {
        await fastify.close();
    }));
    if (err) throw err;
    console.log('Server listening on localhost:', fastify.server.address().port)
});