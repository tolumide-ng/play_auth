use crate::settings::app::AppSettings;

pub fn get_appsettings() -> AppSettings {
    AppSettings {
        port: "200".to_string(), 
        env: "test".to_string(), 
        m_cost: 10000, 
        p_cost: 1, 
        t_cost: 1,
        jwt_secret: "sample_jwt_secret".to_string(),
    }
}