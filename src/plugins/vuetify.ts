import {createVuetify} from "vuetify/framework";
import {aliases, mdi} from "vuetify/iconsets/mdi";
import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'

export default createVuetify({
    icons: {
        defaultSet: 'mdi',
        aliases,
        sets: {
            mdi
        }
    }
})