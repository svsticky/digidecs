use futures_util::future::join_all;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;

pub async fn get_local_v4() -> color_eyre::Result<Ipv4Addr> {
    let potential_addrs = nix::ifaddrs::getifaddrs()?
        // Remove loopback
        .filter_map(|iface| iface.address)
        .filter_map(|addr| addr.as_sockaddr_in().map(|addr4| addr4.ip()))
        .filter(|addr| !addr.is_loopback() && !addr.is_link_local())
        .collect::<Vec<_>>();

    // As we cannot determine if the address can reach the internet just by the address alone, try connecting over TCP
    let connectable_addrs = join_all(potential_addrs.into_iter().map(|addr| async move {
        let sock = tokio::net::TcpSocket::new_v4()?;
        sock.bind(SocketAddr::V4(SocketAddrV4::new(addr, 0)))?;

        match tokio::time::timeout(
            Duration::from_secs(3),
            sock.connect(SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::from([93, 184, 215, 14]),
                80,
            ))),
        )
            .await
        {
            Ok(stream_r) => stream_r.map(|_| addr),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::TimedOut, e)),
        }
    }))
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    if connectable_addrs.is_empty() {
        Err(color_eyre::eyre::Error::msg(
            "Could not determine local ipv4 address".to_string(),
        ))
    } else {
        Ok(connectable_addrs[0])
    }
}

#[cfg(test)]
mod test {

    // We assume there is always a v4 address available!
    #[tokio::test]
    async fn get_local_ipv4() {
        let ip = super::get_local_v4().await;
        assert!(ip.is_ok());
        let ip = ip.unwrap();
        assert!(!ip.is_loopback());
    }
}
