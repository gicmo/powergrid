<template>
  <div v-if="run" >
    <power-graph :log="run['log']"></power-graph>
    <run-info :run="run"></run-info>
    <system-info :info="sysinfo"></system-info>
  </div>
</template>

<script>
  import RunInfo from './RunInfo'
  import SystemInfo from './SystemInfo'
  import PowerGraph from './PowerGraph'

  export default {
    name: 'Run',
    data () {
      return {
        loading: false,
        run: null,
        error: null
      }
    },
    computed: {
      sysinfo () {
        if (!this.run) {
          return null
        }
        let info = this.run['system-info']
        if (!('hardware' in info)) {
          info['hardware'] = { }
        }
        if (!('software' in info)) {
          info['software'] = { }
        }

        let hw = info['hardware']
        if (!('batteries' in hw)) {
          hw['batteries'] = []
        }
        return info
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
    components: { RunInfo, SystemInfo, PowerGraph }
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
