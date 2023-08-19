

export default {
  "meta": {},
  "id": "_default",
  "_regex": {},
  "_paramKeys": {},
  "file": {
    "path": "src/routes/_module.svelte",
    "dir": "src/routes",
    "base": "_module.svelte",
    "ext": ".svelte",
    "name": "_module"
  },
  "asyncModule": () => import('../src/routes/_module.svelte'),
  "rootName": "default",
  "routifyDir": import.meta.url,
  "children": [
    {
      "meta": {},
      "id": "_default_about_svelte",
      "_regex": {},
      "_paramKeys": {},
      "name": "about",
      "file": {
        "path": "src/routes/about.svelte",
        "dir": "src/routes",
        "base": "about.svelte",
        "ext": ".svelte",
        "name": "about"
      },
      "asyncModule": () => import('../src/routes/about.svelte'),
      "children": []
    },
    {
      "meta": {},
      "id": "_default_auctions",
      "_regex": {},
      "_paramKeys": {},
      "name": "auctions",
      "module": false,
      "file": {
        "path": "src/routes/auctions",
        "dir": "src/routes",
        "base": "auctions",
        "ext": "",
        "name": "auctions"
      },
      "children": [
        {
          "meta": {
            "dynamic": true
          },
          "id": "_default_auctions__auction_",
          "_regex": {},
          "_paramKeys": {},
          "name": "[auction]",
          "module": false,
          "file": {
            "path": "src/routes/auctions/[auction]",
            "dir": "src/routes/auctions",
            "base": "[auction]",
            "ext": "",
            "name": "[auction]"
          },
          "children": [
            {
              "meta": {
                "dynamic": true
              },
              "id": "_default_auctions__auction___auction_item_",
              "_regex": {},
              "_paramKeys": {},
              "name": "[auction-item]",
              "module": false,
              "file": {
                "path": "src/routes/auctions/[auction]/[auction-item]",
                "dir": "src/routes/auctions/[auction]",
                "base": "[auction-item]",
                "ext": "",
                "name": "[auction-item]"
              },
              "children": [
                {
                  "meta": {},
                  "id": "_default_auctions__auction___auction_item__index_svelte",
                  "_regex": {},
                  "_paramKeys": {},
                  "name": "index",
                  "file": {
                    "path": "src/routes/auctions/[auction]/[auction-item]/index.svelte",
                    "dir": "src/routes/auctions/[auction]/[auction-item]",
                    "base": "index.svelte",
                    "ext": ".svelte",
                    "name": "index"
                  },
                  "asyncModule": () => import('../src/routes/auctions/[auction]/[auction-item]/index.svelte'),
                  "children": []
                }
              ]
            },
            {
              "meta": {},
              "id": "_default_auctions__auction__index_svelte",
              "_regex": {},
              "_paramKeys": {},
              "name": "index",
              "file": {
                "path": "src/routes/auctions/[auction]/index.svelte",
                "dir": "src/routes/auctions/[auction]",
                "base": "index.svelte",
                "ext": ".svelte",
                "name": "index"
              },
              "asyncModule": () => import('../src/routes/auctions/[auction]/index.svelte'),
              "children": []
            }
          ]
        },
        {
          "meta": {},
          "id": "_default_auctions_index_svelte",
          "_regex": {},
          "_paramKeys": {},
          "name": "index",
          "file": {
            "path": "src/routes/auctions/index.svelte",
            "dir": "src/routes/auctions",
            "base": "index.svelte",
            "ext": ".svelte",
            "name": "index"
          },
          "asyncModule": () => import('../src/routes/auctions/index.svelte'),
          "children": []
        }
      ]
    },
    {
      "meta": {},
      "id": "_default_blog",
      "_regex": {},
      "_paramKeys": {},
      "name": "blog",
      "module": false,
      "file": {
        "path": "src/routes/blog",
        "dir": "src/routes",
        "base": "blog",
        "ext": "",
        "name": "blog"
      },
      "children": [
        {
          "meta": {
            "dynamic": true
          },
          "id": "_default_blog__article__svelte",
          "_regex": {},
          "_paramKeys": {},
          "name": "[article]",
          "file": {
            "path": "src/routes/blog/[article].svelte",
            "dir": "src/routes/blog",
            "base": "[article].svelte",
            "ext": ".svelte",
            "name": "[article]"
          },
          "asyncModule": () => import('../src/routes/blog/[article].svelte'),
          "children": []
        }
      ]
    },
    {
      "meta": {},
      "id": "_default_contact_svelte",
      "_regex": {},
      "_paramKeys": {},
      "name": "contact",
      "file": {
        "path": "src/routes/contact.svelte",
        "dir": "src/routes",
        "base": "contact.svelte",
        "ext": ".svelte",
        "name": "contact"
      },
      "asyncModule": () => import('../src/routes/contact.svelte'),
      "children": []
    },
    {
      "meta": {},
      "id": "_default_index_svelte",
      "_regex": {},
      "_paramKeys": {},
      "name": "index",
      "file": {
        "path": "src/routes/index.svelte",
        "dir": "src/routes",
        "base": "index.svelte",
        "ext": ".svelte",
        "name": "index"
      },
      "asyncModule": () => import('../src/routes/index.svelte'),
      "children": []
    },
    {
      "meta": {
        "dynamic": true,
        "dynamicSpread": true
      },
      "_regex": {},
      "_paramKeys": {},
      "name": "[...404]",
      "file": {
        "path": ".routify/components/[...404].svelte",
        "dir": ".routify/components",
        "base": "[...404].svelte",
        "ext": ".svelte",
        "name": "[...404]"
      },
      "asyncModule": () => import('./components/[...404].svelte'),
      "children": []
    }
  ]
}