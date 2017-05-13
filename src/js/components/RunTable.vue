<template>
  <table class="table table-sm table-hover">
    <thead>
      <tr>
        <th>Action</th>
        <th>Model</th>
        <th>GNOME</th>
        <th>Test</th>
        <th>Power</th>
        <th>Est. Life</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="entry in runs">
        <th scope="row">
          <router-link :to="{ path: '/runs/' + entry.id }">
            <i class="fa fa-area-chart" aria-hidden="true"></i>
          </router-link>
        </a></th>
        <td>{{ entry.model }}</td>
        <td>{{ entry.gnome}} </td>
        <td>{{ entry.test_name }}</td>
        <td>{{ entry.est_power }}</td>
        <td>{{ entry.est_life }}</td>
      </tr>
    </tbody>
  </table>

</template>

<script>
  export default {
    name: 'overview',
    data () {
      return {
        loading: false,
        runs: [],
        error: null
      }
    },
    created () {
      this.update()
    },
    methods: {
      update () {
        this.error = null
        this.runs = []
        this.loading = true

        this.$http.get('/api/runs').then(response => {
          this.runs = response.body
        }, response => {
          console.log(response)
        }).then(
          this.loading = false
        )
      }
    }
  }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h1, h2 {
  font-weight: normal;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  display: inline-block;
  margin: 0 10px;
}

a {
  color: #42b983;
}
</style>
