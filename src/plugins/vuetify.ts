import {createVuetify} from "vuetify/framework";
import {aliases, mdi} from "vuetify/iconsets/mdi";

export default createVuetify({
    icons: {
        defaultSet: 'mdi',
        aliases,
        sets: {
            mdi
        }
    }
})