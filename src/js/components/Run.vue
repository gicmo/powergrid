<template>
  <div v-if="run" >
    <system-info :info="run['system-info']"></system-info>
  </div>
</template>

<script>
  import SystemInfo from './SystemInfo'

  export default {
    name: 'Run',
    data () {
      return {
        loading: false,
        run: null,
        error: null
      }
    },
    mounted () {
      this.update()
    },
    watch: {
      '$route': 'update'
    },
    methods: {
      update () {
        this.error = null
        this.runs = []
        this.loading = true

        const runId = this.$route.params.id

        this.$http.get(`/api/runs/${runId}`).then(resp => {
          this.run = resp.body
        }, err => {
          console.log(err)
        }).then(
          this.loading = false
        )
      }
    },
    components: { SystemInfo }
  }
</script>

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
